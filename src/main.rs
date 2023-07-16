#[macro_use] extern crate rocket;

use std::collections::HashMap;

struct UrlShortener{
    url_map: HashMap<String, String>,
}

impl UrlShortener{
    fn new() -> UrlShortener{
        UrlShortener{
            url_map: HashMap::new(),
        }
    }
}

#[get("/<long_url>")]
fn index(long_url: &str) -> String{
    UrlShortener::new();
    format!("Hello, {}!", long_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}