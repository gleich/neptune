use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use printpdf::*;

pub const WIDTH: f64 = 445.0;
pub const HEIGHT: f64 = 594.0;

/// Only works with TTF fonts
pub struct Fonts {
	pub computer_modern_regular: IndirectFontRef,
	pub computer_modern_italic: IndirectFontRef,
}

pub struct Images {}

pub enum FontWeight {
	Regular,
	Italic,
	Bold,
}

pub struct Properties {
	pub document: PdfDocumentReference,
	pub first_page_index: PdfPageIndex,
	pub first_page_reference: PdfPageReference,
	pub black_layer: PdfLayerReference,
	pub white_layer: PdfLayerReference,
}

lazy_static! {
	static ref ASSETS_FOLDER: PathBuf = PathBuf::from("assets");
}

impl Fonts {
	pub fn new(doc: &PdfDocumentReference) -> Result<Self> {
		let computer_modern = String::from("Computer Modern");
		Ok(Self {
			computer_modern_italic: doc.add_external_font(File::open(
				ASSETS_FOLDER
					.join(&computer_modern)
					.join(FontWeight::Italic.to_string()),
			)?)?,
			computer_modern_regular: doc.add_external_font(File::open(
				ASSETS_FOLDER
					.join(&computer_modern)
					.join(FontWeight::Regular.to_string()),
			)?)?,
		})
	}
}

impl Images {
	pub fn load_logo() -> Result<Image> {
		Self::load_image(&ASSETS_FOLDER.join("logo.jpg")).context("Failed to load logo")
	}

	// image must be JPEG
	pub fn load_image(path: &PathBuf) -> Result<Image> {
		let mut file = File::open(path).context(format!("Failed to load {}", path.display()))?;
		Ok(Image::try_from(
			image_crate::codecs::jpeg::JpegDecoder::new(&mut file)
				.context(format!("Failed to decode png for {}", path.display()))?,
		)
		.context(format!(
			"Failed to convert codecs to Image for {}",
			path.display()
		))?)
	}
}

impl Properties {
	pub fn new<T: Into<String>>(name: T) -> Self {
		let (doc, first_page_index, layer1) =
			PdfDocument::new(name, Mm(WIDTH), Mm(HEIGHT), "black");
		let first_page_reference = doc.get_page(first_page_index);
		let black_layer = first_page_reference.get_layer(layer1);
		let white_layer = first_page_reference.add_layer("white");

		black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
		white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

		Self {
			first_page_index,
			first_page_reference,
			black_layer,
			white_layer,
			document: doc,
		}
	}
}

impl ToString for FontWeight {
	fn to_string(&self) -> String {
		match self {
			FontWeight::Bold => String::from("bold.ttf"),
			FontWeight::Regular => String::from("regular.ttf"),
			FontWeight::Italic => String::from("regular.ttf"),
		}
	}
}
