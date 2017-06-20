extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
use futures::{Future, Stream};
use hyper::Client;
use hyper::Uri;
use hyper::Chunk;
use hyper::error::UriError;
use tokio_core::reactor::Core;
use std::str;
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
}

#[derive(Serialize, Deserialize)]
struct Weather {
    main: String,
}

fn open_weather_uri(api_key: String) -> Result<Uri, UriError> {
    format!(
        "{}{}{}",
        "http://api.openweathermap.org/data/2.5/",
        "weather?q=Porto+Alegre&APPID=",
        api_key
    ).parse()
}

fn request_weather(core: &mut Core, uri: Uri) -> Result<Chunk, hyper::Error> {
    let client = Client::new(&core.handle());
    let work = client.get(uri).and_then(|res| res.body().concat2());
    core.run(work)
}

fn main() {
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let uri = open_weather_uri(api_key).unwrap();
    let mut core = Core::new().unwrap();
    let request = request_weather(&mut core, uri).unwrap();
    let json = str::from_utf8(&request).unwrap();
    let weather: WeatherResponse = serde_json::from_str(json).unwrap();

    println!("Current weather: {}", weather.weather[0].main);
}
