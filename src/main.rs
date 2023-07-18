mod models;
mod test;

#[macro_use] extern crate rocket;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use crate::models::Database;
use rocket::serde::json::Json;
use rocket::http::Status;


#[derive(Serialize, Deserialize)]
pub struct UrlShortener{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    long_url: String,
    short_url: String,

}

#[derive(Serialize, Deserialize)]
pub struct Response{
    pub id: Option<ObjectId>,
    pub long_url: String,
    pub short_url: String,
}

impl UrlShortener{
    fn new(long_url: String) -> UrlShortener{
        UrlShortener{
            id: None,
            long_url: long_url.clone(),
            short_url: Self::shorten_url(&long_url),
        }
    }

    fn shorten_url(long_url: &str) -> String{
        let hashed_url = Self::hash_url(long_url);
        hashed_url[0..8].to_string()
    }

    fn hash_url(long_url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(long_url);
        let hashed_url = hasher.finalize();

        general_purpose::STANDARD.encode(hashed_url)
    }


    }


#[get("/<long_url>")]
fn index(long_url: String) -> Result<Json<Response>, Status> {
    let url_shortener = UrlShortener::new(long_url.clone());
    let database = match Database::new("mongodb://localhost:27017", "url_shortener", "urls") {
        Ok(database) => database,
        Err(_) => return Err(Status::InternalServerError),
    };

    let details = match database.check_or_insert(&url_shortener) {
        Ok(details) => details,
        Err(_) => return Err(Status::InternalServerError),
    };

    let response = Response{
        id: details,
        long_url: long_url.clone(),
        short_url: url_shortener.short_url,
    };

    Ok(Json(response))

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}