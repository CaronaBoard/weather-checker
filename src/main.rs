extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate lettre;
use futures::{Future, Stream};
use hyper::Client;
use hyper::Uri;
use hyper::Chunk;
use hyper::error::UriError;
use tokio_core::reactor::Core;
use std::str;
use std::env;
use lettre::email::EmailBuilder;
use lettre::email::Email;
use lettre::transport::smtp;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

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
        "http://api.openweathermap.org/data/2.5/weather",
        "?q=Porto+Alegre&APPID=",
        api_key
    ).parse()
}

fn request_weather(core: &mut Core, uri: Uri) -> Result<Chunk, hyper::Error> {
    let client = Client::new(&core.handle());
    let work = client.get(uri).and_then(|res| res.body().concat2());
    core.run(work)
}

fn build_email(email_to: &str) -> Result<Email, lettre::email::error::Error> {
    EmailBuilder::new()
        .to(email_to)
        .from(("hello@caronaboard.com", "CaronaBoard"))
        .body(
            r#"
Está chovendo hoje em Porto Alegre, não vai querer se molhar né?

Pegue uma carona com o Caronaboard! https://caronaboard.com/

ps.: o projeto ainda está no início, agradecemos muito os feedbacks!

---
Este é um email automático enviado por um código feito em Rust.
Quer aprender mais? Contribua! https://github.com/caronaBoard
        "#,
        )
        .subject("Está chovendo! Que tal usar o Caroaboard?")
        .build()
}

fn send_email(
    email: Email,
    smtp_pass: &str,
) -> Result<smtp::response::Response, smtp::error::Error> {
    let mut mailer = SmtpTransportBuilder::new(("smtp.mailgun.org", SUBMISSION_PORT))?
        .credentials("postmaster@caronaboard.com", smtp_pass)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .build();

    mailer.send(email)
}

fn main() {
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let email_to = env::var("EMAIL_TO").expect("EMAIL_TO not set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS not set");

    let uri = open_weather_uri(api_key).unwrap();
    let mut core = Core::new().unwrap();
    let request = request_weather(&mut core, uri).unwrap();
    let json = str::from_utf8(&request).unwrap();
    let weather: WeatherResponse = serde_json::from_str(json).unwrap();

    if weather.weather[0].main == "Rain" {
        println!("It is raining! The email will be sent.");

        let email = build_email(&email_to).unwrap();
        send_email(email, &smtp_pass).unwrap();

        println!("Done!");
    } else {
        println!(
            "Not raining, no emails were sent. Current weather: {}",
            weather.weather[0].main
        );
    }
}
