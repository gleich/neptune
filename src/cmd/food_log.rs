use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local};
use genpdf::style::Style;
use genpdf::{elements, Alignment, Document, Element, Margins, Position, Scale};
use ordinal::Ordinal;
use task_log::task;

use crate::document;

pub fn cli_run() {
	let now = Local::now();
	let filename = Ordinal(now.day()).to_string();
	let document = new_document(now, &filename).expect("Failed to create document");
	document::upload(
		document,
		filename,
		PathBuf::from("Food Logs").join(now.format("%B").to_string()),
		false,
	)
	.expect("Failed to upload document");
}

pub fn new_document(now: DateTime<Local>, filename: &String) -> Result<Document> {
	let mut document = task("Creating document", || document::new(filename))
		.context("Failed to create document")?;
	task("Writing pages", || {
		let food_log_img = elements::Image::from_path("assets/food_log.jpg")
			.expect("Failed to load food log template image")
			.with_position(Position::new(0, -12))
			.with_scale(Scale::new(2.1, 2.1));
		document.push(food_log_img);
		document.push(
			elements::Paragraph::new(now.format("Created at %r").to_string())
				.aligned(Alignment::Right)
				.styled(Style::new().with_font_size(18).bold())
				.padded(Margins::trbl(17, 50, 0, 0)),
		);
		document.push(
			elements::Paragraph::new(
				now.format(&format!("%A the {} of %B", filename))
					.to_string(),
			)
			.aligned(Alignment::Right)
			.styled(Style::new().with_font_size(35).bold())
			.padded(Margins::trbl(0, 50, 0, 0)),
		);
	});
	Ok(document)
}
