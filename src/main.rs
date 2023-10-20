
#[macro_use] extern crate rocket;

use rocket::Config;
use std::net::IpAddr;
use std::str::FromStr;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[launch]
fn rocket() -> _ {
    let port_str = env::var("PORT").unwrap_or_else(|_| String::from("8000"));
    let port = port_str.parse::<u16>().expect("Failed to parse PORT as u16");

    rocket::build()
        .mount("/", routes![index])
        .configure(Config {
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            port,
            ..Config::default()
        })
}