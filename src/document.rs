use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context, Result};

use crate::resources;

pub struct Document {
	pub name: String,
	pub folder: PathBuf,
	pub filename: String,
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
			filename: format!("{}.pdf", name),
			name,
			folder: PathBuf::from(folder.into()),
			fonts: resources::Fonts::new(&properties.document).context("Failed to load fonts")?,
			properties,
		})
	}

	pub fn save(self) -> Result<()> {
		let tmp_folder = env::temp_dir()
			.join("com.mattgleich.neptune")
			.join(rand::random::<u8>().to_string());

		fs::create_dir_all(&tmp_folder).context("Failed to create temporary folder")?;
		self.properties
			.document
			.save(&mut BufWriter::new(
				File::create(tmp_folder.join(self.filename)).context("Failed to create file")?,
			))
			.context("Failed to save document")?;
		Ok(())
	}

	pub fn debug_save(self) -> Result<()> {
		self.properties
			.document
			.save(&mut BufWriter::new(
				File::create("test.pdf").context("Failed to create file")?,
			))
			.context("Failed to save document")?;
		Ok(())
	}
}
