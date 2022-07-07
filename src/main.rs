#[macro_use]
extern crate rocket;

use rocket::build;

mod cmd;
mod document;
mod write;
mod auth;
mod result;

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![cmd::daily_log::route])
}
