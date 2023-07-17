use mongodb::{
    bson::{extjson::de::Error},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use crate::UrlShortener;

pub struct Database{
    pub(crate) client: Client,
    pub(crate) collection: Collection<UrlShortener>,
}

impl Database{
    pub fn new() -> Database{
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        let collection = client.database("url_shortener").collection("urls");
        Database{
            client,
            collection,
        }
    }
}