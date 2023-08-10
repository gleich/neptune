use std::path::Path;

use anyhow::{Context, Result};
use genpdf::{fonts, Document, Size};

pub fn new(name: &String) -> Result<Document> {
	let name: String = name.into();
	let default_font =
		fonts::from_files("assets/fonts/", "Inter", None).expect("Failed to load default font");
	let mut core_document = genpdf::Document::new(default_font);
	core_document.set_title(&name);
	core_document.set_minimal_conformance();
	core_document.set_paper_size(Size::new(445, 594));
	Ok(core_document)
}
