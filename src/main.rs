#[macro_use]
extern crate rocket;

use rocket::{launch, Config};

mod document;
mod endpoints;
mod rmapi;
mod templates;

#[launch]
fn rocket() -> _ {
	// rmapi::setup().expect("Failed to setup RMAPI");
	let config = Config::figment().merge(("address", "0.0.0.0"));
	rocket::custom(config).mount("/", routes![endpoints::note])
}
