use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

use anyhow::{ensure, Context, Result};
use genpdf::{fonts, Document, Size};
use task_log::task;

pub fn new<T: Into<String>>(name: T) -> Result<Document> {
	let name: String = name.into();
	let default_font =
		fonts::from_files("assets/fonts/", "Inter", None).expect("Failed to load default font");
	let mut core_document = genpdf::Document::new(default_font);
	core_document.set_title(&name);
	core_document.set_minimal_conformance();
	core_document.set_paper_size(Size::new(445, 594));
	Ok(core_document)
}

#[allow(dead_code)]
pub fn debug_save(document: Document) -> Result<()> {
	let uncompressed_filename = "debug_uncompressed.pdf";
	task("Saving debug version of PDF", || -> Result<()> {
		save(document, uncompressed_filename)
	})?;
	task(
		"Compressing output PDF with ghostscript",
		|| -> Result<()> { compress(uncompressed_filename, "debug.pdf") },
	)?;
	exit(0);
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
		.arg(format!("-sOutputFile={filename}"))
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
	Command::new("rmapi")
		.arg("mkdir")
		.arg(&folder)
		.output()
		.context("Failed to spawn process to make parent directory")?;

	Command::new("rmapi")
		.arg("put")
		.arg(&filename)
		.arg(&folder)
		.output()
		.context("Failed to spawn process to upload document")?;

	fs::remove_file(filename)
		.context("Failed to remove file after upload")
		.context("Failed to delete pdf")?;

	Ok(())
}
