#[macro_use]
extern crate rocket;

use rocket::build;

mod cli;
mod cmd;
mod document;
mod write;
mod auth;

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![cmd::daily_log::route])
}
