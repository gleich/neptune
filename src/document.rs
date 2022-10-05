use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use genpdf::{fonts, Document, Size};
use task_log::task;

pub fn new<T: Into<String>>(name: T) -> Result<Document> {
	let name: String = name.into();
	let default_font = fonts::from_files("assets/fonts/", "Computer Modern", None)
		.expect("Failed to load default font");
	let mut core_document = genpdf::Document::new(default_font);
	core_document.set_title(&name);
	core_document.set_minimal_conformance();
	core_document.set_paper_size(Size::new(445, 594));
	Ok(core_document)
}

#[allow(dead_code)]
pub fn debug_save(document: Document) -> Result<()> {
	let uncompressed_filename = "debug_uncompressed.pdf";
	task("Saving PDF", || -> Result<()> {
		save(document, uncompressed_filename)
	})?;
	task(
		"Compressing output PDF with ghostscript",
		|| -> Result<()> { compress(uncompressed_filename, "debug.pdf") },
	)?;
	Ok(())
}

pub fn save<T: Into<String>>(document: Document, uncompressed_filename: T) -> Result<()> {
	document
		.render_to_file(uncompressed_filename.into())
		.context("Failed to render to file")?;
	Ok(())
}

pub fn compress(uncompressed_filename: &str, filename: &str) -> Result<()> {
	let status = Command::new("gs")
		.args([
			"-q",
			"-dBATCH",
			"-dSAFER",
			"-dNOPAUSE",
			"-sDEVICE=pdfwrite",
			"-dCompatibilityLevel=1.4",
			"-dPDFSETTINGS=/ebook",
			"-dAutoRotatePages=/None",
			"-dColorImageDownsampleType=/Bicubic",
			"-dColorImageResolution=135",
			"-dGrayImageDownsampleType=/Bicubic",
			"-dGrayImageResolution=135",
			"-dMonoImageDownsampleType=/Bicubic",
			"-dMonoImageResolution=135",
		])
		.arg(format!("-sOutputFile={}", filename))
		.arg(uncompressed_filename)
		.spawn()
		.context("Failed to run compression ghostscript")?
		.wait()
		.context("Failed to wait for ghostscript compression to complete")?;
	ensure!(status.success());
	fs::remove_file(uncompressed_filename).context("Failed to remove uncompressed PDF")?;
	Ok(())
}

pub fn upload<T: Into<String>>(filename: T, folder: PathBuf) -> Result<()> {
	let filename: String = filename.into();
	let mut process = Command::new("rmapi")
		.arg("mkdir")
		.arg(&folder)
		.spawn()
		.context("Failed to spawn process to make parent directory")?;
	process.stdout.take();
	let mut status = process.wait().context("Failed to make parent directory")?;
	ensure!(status.success());

	process = Command::new("rmapi")
		.arg("put")
		.arg(&filename)
		.arg(&folder)
		.spawn()
		.context("Failed to spawn process to upload document")?;
	process.stdout.take();
	status = process.wait().context("Failed to make parent directory")?;
	ensure!(status.success());

	fs::remove_file(filename)
		.context("Failed to remove file after upload")
		.context("Failed to delete pdf")?;

	Ok(())
}
