
#[macro_use] extern crate rocket;

use rocket::Config;
use std::net::IpAddr;
use std::str::FromStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .configure(Config {
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            port: 8000,
            ..Config::default()
        })
}