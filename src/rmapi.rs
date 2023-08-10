use std::{
	env, fs,
	path::{Path, PathBuf},
	process::Command,
};

use anyhow::{ensure, Context, Result};
use genpdf::Document;

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

pub fn upload(doc: Document, name: &String, folder: &Path) -> Result<()> {
	let uncompressed_filename = format!("{} uncompressed.pdf", &name);
	let compressed_filename = format!("{}.pdf", &name);

	doc.render_to_file(&uncompressed_filename)
		.context("Failed to render document to file")?;

	// compressing PDF using ghostscript
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
		.arg(format!("-sOutputFile={compressed_filename}"))
		.arg(&uncompressed_filename)
		.spawn()
		.context("Failed to run compression ghostscript")?
		.wait()
		.context("Failed to wait for ghostscript compression to complete")?;
	ensure!(status.success());
	fs::remove_file(uncompressed_filename).context("Failed to remove uncompressed PDF")?;

	// uploading using rmapi
	Command::new("rmapi")
		.arg("mkdir")
		.arg(folder.to_str().unwrap().trim_end_matches('/'))
		.output()
		.context("Failed to spawn process to make parent directory")?;
	Command::new("rmapi")
		.arg("put")
		.arg(&compressed_filename)
		.arg(&folder)
		.output()
		.context("Failed to spawn process to upload document")?;
	fs::remove_file(compressed_filename).context("Failed to remove file after upload")?;

	Ok(())
}
