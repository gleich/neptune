use std::fs;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use genpdf::fonts;
use task_log::task;

pub struct Document {
	pub name: String,
	pub core_document: genpdf::Document,
}

impl Document {
	pub fn new<T: Into<String>>(name: T) -> Result<Self> {
		let name: String = name.into();
		let default_font = fonts::from_files("assets/fonts/", "Computer Modern", None)
			.expect("Failed to load default font");
		let mut core_document = genpdf::Document::new(default_font);
		core_document.set_title(&name);
		core_document.set_minimal_conformance();
		Ok(Self {
			name,
			core_document,
		})
	}

	pub fn debug_save(self) -> Result<()> {
		let uncompressed_filename = "debug_uncompressed.pdf";
		task("Saving PDF", || -> Result<()> {
			self.core_document
				.render_to_file(uncompressed_filename)
				.context("Failed to render to file")?;
			Ok(())
		})?;
		task(
			"Compressing output PDF with ghostscript",
			|| -> Result<()> {
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
					.arg("-sOutputFile=debug.pdf")
					.arg(uncompressed_filename)
					.spawn()
					.context("Failed to run compression ghostscript")?
					.wait()
					.context("Failed to wait for ghostscript compression to complete")?;
				ensure!(status.success());
				Ok(())
			},
		)?;
		fs::remove_file(uncompressed_filename).context("Failed to remove uncompressed PDF")?;
		Ok(())
	}
}
