#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Hello {
    name: String,
}

#[get("/", format = "json")]
fn hello() -> Json<Hello> {
    Json(Hello { name: "Mno".into() })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}