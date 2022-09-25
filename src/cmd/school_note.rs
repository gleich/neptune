use std::path::PathBuf;

use anyhow::Result;
use chrono::{Datelike, Local};
use genpdf::style::Style;
use genpdf::{elements, Alignment, Element, Margins, Position, Scale};
use ordinal::Ordinal;
use task_log::task;

use crate::document;

pub enum Class {
	MATH171,
	MEDG101,
}

impl ToString for Class {
	fn to_string(&self) -> String {
		match self {
			Self::MATH171 => format!("{}: Calculus A", self.short_name()),
			Self::MEDG101 => format!("{}: Human Biology 1", self.short_name()),
		}
	}
}

impl Class {
	pub fn short_name(&self) -> String {
		match self {
			Self::MATH171 => String::from("MATH 171"),
			Self::MEDG101 => String::from("MEDG 110"),
		}
	}
}

pub fn raw_run<T: Into<String>>(name: T, class: Class, folder: T) -> Result<()> {
	let mut document = task("Creating document", || {
		document::new("hello world").expect("Failed to create document")
	});
	let name: String = name.into();

	task("Creating title page", || {
		document.push(
			elements::Image::from_path("assets/logo.jpg")
				.expect("Failed to load logo")
				.with_position(Position::new(165, 150))
				.with_scale(Scale::new(0.9, 0.9)),
		);
		document.push(
			elements::Paragraph::new(&name)
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(50))
				.padded(Margins::trbl(285, 70, 10, 70)),
		);
		document.push(
			elements::Paragraph::new(class.to_string())
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(25).italic())
				.padded(Margins::trbl(0, 0, 35, 0)),
		);
		document.push(
			elements::Paragraph::new("Matt Gleich")
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(35)),
		);
		let now = Local::now();
		document.push(
			elements::Paragraph::new(
				now.format(&format!("%A, %B %e{}, %Y", Ordinal(now.day()).suffix()))
					.to_string(),
			)
			.aligned(Alignment::Center)
			.styled(Style::new().with_font_size(30).italic()),
		);
		document.push(
			elements::Paragraph::new(now.format("%l:%M:%S %p").to_string())
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(30).italic()),
		);
	});

	task("Writing pages", || {
		let note_img = elements::Image::from_path("assets/note.jpg")
			.expect("Failed to load logo")
			.with_position(Position::new(0, -12))
			.with_scale(Scale::new(2.1, 2.1));
		for _ in 1..5 {
			document.push(elements::PageBreak::new());
			document.push(note_img.clone());
			document.push(
				elements::Paragraph::new(format!("{} | {} | Matt Gleich", name, class.to_string()))
					.aligned(Alignment::Center)
					.styled(Style::new().with_font_size(25).italic())
					.padded(10),
			);
		}
	});

	let uncompressed_filename = format!("{} uncompressed.pdf", name);
	let filename = format!("{}.pdf", name);
	task("Saving document", || {
		document::save(document, &uncompressed_filename).expect("Failed to save document");
	});
	task("Compressing PDF file", || {
		document::compress(&uncompressed_filename, &filename).expect("Failed to compress document");
	});
	task("Uploading file", || {
		document::upload(
			filename,
			PathBuf::from("Notes")
				.join("College")
				.join(class.short_name())
				.join(folder.into()),
		)
		.expect("Failed to upload document");
	});

	Ok(())
}
