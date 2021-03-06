#[macro_use]
extern crate rocket;

use std::{env, fs, path::PathBuf};

use anyhow::Result;
use rocket::Config;

mod auth;
mod cmd;
mod document;
mod result;
mod write;

#[launch]
fn rocket() -> _ {
    setup_rmapi().expect("Failed to setup rmapi");
    let config = Config::figment().merge(("address", "0.0.0.0"));
    rocket::custom(config).mount("/", routes![cmd::daily_log::route, cmd::note::route])
}

fn setup_rmapi() -> Result<()> {
    let device_token = env::var("RMAPI_DEVICE_TOKEN")?;
    let user_token = env::var("RMAPI_USER_TOKEN")?;
    let config_folder = PathBuf::from("/root/.config/rmapi");
    fs::create_dir_all(&config_folder)?;
    fs::write(
        config_folder.join("rmapi.conf"),
        format!("devicetoken: {}\nusertoken: {}\n", device_token, user_token),
    )?;
    println!("Setup RMAPI");
    Ok(())
}
