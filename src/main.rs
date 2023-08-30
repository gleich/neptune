#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast::channel;
use rocket::{launch, Config};

mod document;
mod endpoints;
mod rmapi;
mod templates;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Status {
	pub message: String,
	pub progress: u32,
}

#[launch]
fn rocket() -> _ {
	// rmapi::setup().expect("Failed to setup RMAPI");
	let config = Config::figment().merge(("address", "0.0.0.0"));
	rocket::custom(config)
		.manage(channel::<Status>(1024).0)
		.mount("/", routes![endpoints::note])
}
