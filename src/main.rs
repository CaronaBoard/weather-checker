extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use std::str;

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

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = format!(
        "{}{}",
        "http://api.openweathermap.org/data/2.5/",
        "weather?q=Porto+Alegre&APPID=APPKEY"
    ).parse()
        .unwrap();

    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2()
    });
    let data = core.run(work).unwrap();
    let json = str::from_utf8(&data).unwrap();

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let w: WeatherResponse = serde_json::from_str(json).unwrap();

    // Do things just like with any other Rust data structure.
    println!("Current weather: {}", w.weather[0].main);
}
