use mongodb::{
    bson::{extjson::de::Error},
    results::InsertOneResult,
    sync::{Client, Collection},
    error::Result as MongoResult,
};
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;

use crate::UrlShortener;

pub struct Database{
    pub(crate) client: Client,
    pub(crate) collection: Collection<UrlShortener>,
}

impl Database{
    pub fn new(uri: &str, db_name: &str, collection_name: &str) -> MongoResult<Database>{
        let client = Client::with_uri_str(uri)?;
        let collection = client.database(db_name).collection(collection_name);
        Ok(Database{client, collection})
    }

    pub fn check_or_insert(&self, url: &UrlShortener) -> MongoResult<Option<ObjectId>> {
        let filter = doc! {"long_url": &url.long_url, "short_url": &url.short_url};

        if let Some(existing) = self.collection.find_one(filter.clone(), None)? {
            // The URL is already in the database, so return the ObjectId.
            return Ok(existing.id);
        }

        // The URL is not in the database, so insert it and return the inserted_id.
        let result = self.collection.insert_one(url, None)?;
        match result.inserted_id {
            Bson::ObjectId(id) => Ok(Some(id)),
            _ => Ok(None),
        }
    }
}