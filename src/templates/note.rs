use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use genpdf::style::{Color, Style};
use genpdf::{elements, Alignment, Document, Element, Margins, Position, Scale};
use ordinal::Ordinal;
use task_log::task;

use crate::document;

pub struct Note {
	pub name: String,
	pub subject: String,
}

impl Note {
	pub fn create(&self) -> Result<Document> {
		let mut doc = document::new(&self.name).context("Failed to create document for note")?;
		self.write_content(&mut doc)
			.context("Failed to add main pages")?;
		Ok(doc)
	}

	pub fn write_content(&self, doc: &mut Document) -> Result<()> {
		task("Writing main page", || -> Result<()> {
			let text_color = Color::Rgb(255, 255, 255);
			let now = Local::now();
			let use_small_font = self.name.len() > 34;
			let note_img = elements::Image::from_path("/Users/matt/src/neptune/assets/note.jpg")
				.context("Failed to load note template image")?
				.with_position(Position::new(0, -12))
				.with_scale(Scale::new(2.1, 2.1));
			doc.push(note_img.clone());
			doc.push(
				elements::Paragraph::new(&self.name)
					.aligned(Alignment::Left)
					.styled(
						Style::new()
							.with_font_size(if use_small_font { 40 } else { 60 })
							.bold()
							.with_color(text_color),
					)
					.padded(Margins::trbl(5, 0, 0, 18)),
			);
			doc.push(
				elements::Paragraph::new(&self.subject)
					.aligned(Alignment::Left)
					.styled(
						Style::new()
							.with_font_size(17)
							.bold()
							.with_color(text_color),
					)
					.padded(Margins::trbl(5, 0, 0, 18)),
			);
			doc.push(
				elements::Paragraph::new(now.format("© Matt Gleich %Y").to_string())
					.aligned(Alignment::Left)
					.styled(
						Style::new()
							.with_font_size(17)
							.bold()
							.with_color(text_color),
					)
					.padded(Margins::trbl(
						if use_small_font { 537 } else { 530 },
						0,
						0,
						20,
					)),
			);
			doc.push(
				elements::Paragraph::new(
					now.format(&format!("%A %B {}, %Y @ %r", Ordinal(now.day())))
						.to_string(),
				)
				.aligned(Alignment::Right)
				.styled(
					Style::new()
						.with_font_size(17)
						.bold()
						.with_color(text_color),
				)
				.padded(Margins::trbl(-7, 23, 0, 0)),
			);
			Ok(())
		})
	}
}
