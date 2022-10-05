use std::fmt::Display;
use std::fs;

use anyhow::{Context, Result};
use directories::UserDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Options {
	pub classes: Vec<Class>,
	pub books: Vec<Book>,
}

#[derive(Deserialize, Debug)]
pub struct Class {
	pub id: String,
	pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Book {
	pub name: String,
	pub author: String,
}

impl Options {
	pub fn read() -> Result<Self> {
		let dirs = UserDirs::new().context("Failed to get user directories")?;
		let raw_content = fs::read_to_string(
			dirs.home_dir()
				.join(".config")
				.join("neptune")
				.join("options.toml"),
		)
		.context("Failed to read from options file")?;
		let content: Options =
			toml::from_str(&raw_content).context("Failed to parse TOML file for options")?;
		Ok(content)
	}
}

impl Display for Book {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} (by {})", self.name, self.author)
	}
}
