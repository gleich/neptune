use std::path::PathBuf;

use anyhow::{Context, Result};
use genpdf::{elements, Document, Position, Scale};

pub mod note;

pub fn insert_template_image(document: &mut Document, name: String) -> Result<()> {
	let path = PathBuf::from("assets").join(name);
	document.push(
		elements::Image::from_path(&path)
			.context(format!("Failed to load template image: {}", path.display()))?
			.with_position(Position::new(0, -12))
			.with_scale(Scale::new(2.1, 2.1)),
	);
	Ok(())
}
