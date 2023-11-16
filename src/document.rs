use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use genpdf::{fonts, Document, Size};
use task_log::task;

pub fn new(name: &String) -> Result<Document> {
	let name: String = name.into();
	let default_font = fonts::from_files("/Users/matt/src/neptune/assets/fonts/", "Inter", None)
		.expect("Failed to load default font");
	let mut core_document = genpdf::Document::new(default_font);
	core_document.set_title(&name);
	core_document.set_minimal_conformance();
	core_document.set_paper_size(Size::new(445, 594));
	Ok(core_document)
}

pub fn save(name: &String, doc: Document) -> Result<PathBuf> {
	task("Saving PDF", || -> Result<PathBuf> {
		let data_folder = dirs::data_dir()
			.context("Failed to find data directory")?
			.join("neptune");
		let documents_folder = data_folder.join("documents");
		if !data_folder.exists() {
			fs::create_dir_all(data_folder).context("Failed to create data directory")?;
		}
		// clear out documents folder
		if documents_folder.exists() {
			fs::remove_dir_all(&documents_folder).context("Failed to delete documents folder")?;
		}
		fs::create_dir_all(&documents_folder).context("Failed to create documents folder")?;

		let path = documents_folder.join(format!("{}.pdf", name));
		doc.render_to_file(documents_folder.join(format!("{}.pdf", name)))
			.context("Failed to output file to PDF")
			.context("Failed to render content to file")?;
		Ok(path)
	})
}

pub fn open(path: &Path) -> Result<()> {
	task(
		format!(
			"Opening {} in GoodNotes",
			path.file_name().unwrap().to_str().unwrap()
		),
		|| -> Result<()> {
			Command::new("open")
				.arg("-a")
				.arg("GoodNotes.app")
				.arg(path)
				.output()
				.context("Failed to run open command")?;
			Ok(())
		},
	)?;
	Ok(())
}
