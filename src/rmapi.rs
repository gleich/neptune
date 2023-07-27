use std::{env, fs, path::PathBuf};

use anyhow::Result;

pub fn setup() -> Result<()> {
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
