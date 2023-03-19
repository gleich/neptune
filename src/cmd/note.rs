use std::path::PathBuf;

use anyhow::Result;
use chrono::{Datelike, Local};
use genpdf::style::Style;
use genpdf::{elements, Alignment, Element, Margins, Position, Scale};
use ordinal::Ordinal;
use task_log::task;

use crate::document;

pub fn new<T: ToString>(name: T, subject: &str, folder: T) -> Result<()> {
	let name: String = name.to_string();
	let mut document = task("Creating document", || {
		document::new(&name).expect("Failed to create document")
	});

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
				.styled(Style::new().with_font_size(50).bold())
				.padded(Margins::trbl(285, 70, 10, 70)),
		);
		document.push(
			elements::Paragraph::new(subject.to_string())
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(25))
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
				now.format(&format!("%A %B %e{}, %Y", Ordinal(now.day()).suffix()))
					.to_string(),
			)
			.aligned(Alignment::Center)
			.styled(Style::new().with_font_size(30)),
		);
		document.push(
			elements::Paragraph::new(now.format("%l:%M:%S %p").to_string())
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(25)),
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
				elements::Paragraph::new(&name)
					.aligned(Alignment::Right)
					.styled(Style::new().with_font_size(17))
					.padded(Margins::trbl(29, 13, 0, 0)),
			);
			document.push(
				elements::Paragraph::new(subject)
					.aligned(Alignment::Right)
					.styled(Style::new().with_font_size(17))
					.padded(Margins::trbl(0, 13, 0, 0)),
			);
		}
	});

	let uncompressed_filename = format!("{name} uncompressed.pdf");
	let filename = format!("{name}.pdf");

	task("Saving document", || {
		document::save(document, &uncompressed_filename).expect("Failed to save document");
	});
	task("Compressing PDF file", || {
		document::compress(&uncompressed_filename, &filename).expect("Failed to compress document");
	});
	task("Uploading file", || {
		document::upload(filename, PathBuf::from(folder.to_string()))
			.expect("Failed to upload document");
	});

	Ok(())
}
