#[macro_use]
extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, json::Json};
// add delay to simulate a server response
#[get("/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Delayed for {} seconds", seconds)
}

#[get("/")]
fn json() -> Json<String> {
    Json(String::from("Hello, JSON!"))
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/fun")]
fn fun() -> &'static str {
    "Hello, fun!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world, fun])
        .mount("/delay", routes![delay])
        .mount("/json", routes![json])
}
