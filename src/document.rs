use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::resources;

pub struct Document {
	pub name: String,
	pub folder: PathBuf,
	pub fonts: resources::Fonts,
	pub properties: resources::Properties,
}

impl Document {
	pub fn new<T, P>(name: T, folder: P) -> Result<Self>
	where
		T: Into<String>,
		P: Into<String>,
	{
		let name: String = name.into();
		let properties = resources::Properties::new(&name);
		Ok(Self {
			name,
			folder: PathBuf::from(folder.into()),
			fonts: resources::Fonts::new(&properties.document).context("Failed to load fonts")?,
			properties,
		})
	}
}
