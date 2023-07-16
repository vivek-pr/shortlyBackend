#[macro_use] extern crate rocket;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose, alphabet};

struct UrlShortener{
    long_url: String,
    short_url: String,

}

impl UrlShortener{
    fn new() -> UrlShortener{
        UrlShortener{
            long_url: String::from(""),
            short_url: String::from(""),
        }
    }

    fn shorten_url(&mut self, long_url: String) -> String{
        let hashed_url = self.hash_url(long_url);
        let short_url = self.encode_url(hashed_url);
        short_url
    }

    fn hash_url(&self, long_url: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(long_url);
        let hashed_url = hasher.finalize();

        let hashed_url = general_purpose::STANDARD.encode(hashed_url);
        hashed_url
    }


    fn encode_url(&mut self, hashed_url: String) -> String{
        hashed_url[0..8].to_string()
    }

    }


#[get("/<long_url>")]
fn index(long_url: &str) -> String{
    let mut url_shortener = UrlShortener::new();
    let long_url = String::from(long_url);
    let short_url = url_shortener.shorten_url(long_url);
    format!("Hello, {}!", short_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}