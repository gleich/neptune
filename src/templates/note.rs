use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use genpdf::style::Style;
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
		self.add_title_page(&mut doc)
			.context("Failed to load title page")?;
		self.add_main_page(&mut doc)
			.context("Failed to add main pages")?;
		Ok(doc)
	}

	pub fn add_title_page(&self, doc: &mut Document) -> Result<()> {
		task("Writing title page", || -> Result<()> {
			doc.push(
				elements::Image::from_path("/Users/matt/src/neptune/assets/logo.jpg")
					.context("Failed to load logo")?
					.with_position(Position::new(165, 150))
					.with_scale(Scale::new(0.9, 0.9)),
			);
			doc.push(
				elements::Paragraph::new(&self.name)
					.aligned(Alignment::Center)
					.styled(Style::new().with_font_size(50).bold())
					.padded(Margins::trbl(285, 70, 10, 70)),
			);
			doc.push(
				elements::Paragraph::new(self.subject.to_string())
					.aligned(Alignment::Center)
					.styled(Style::new().with_font_size(25))
					.padded(Margins::trbl(0, 0, 35, 0)),
			);
			doc.push(
				elements::Paragraph::new("Matt Gleich")
					.aligned(Alignment::Center)
					.styled(Style::new().with_font_size(35)),
			);
			let now = Local::now();
			doc.push(
				elements::Paragraph::new(
					now.format(&format!("%A %B %e{}, %Y", Ordinal(now.day()).suffix()))
						.to_string(),
				)
				.aligned(Alignment::Center)
				.styled(Style::new().with_font_size(30)),
			);
			doc.push(
				elements::Paragraph::new(now.format("%l:%M:%S %p").to_string())
					.aligned(Alignment::Center)
					.styled(Style::new().with_font_size(25)),
			);
			Ok(())
		})
	}

	pub fn add_main_page(&self, doc: &mut Document) -> Result<()> {
		task("Writing main page", || -> Result<()> {
			let note_img = elements::Image::from_path("/Users/matt/src/neptune/assets/note.jpg")
				.context("Failed to load note template image")?
				.with_position(Position::new(0, -12))
				.with_scale(Scale::new(2.1, 2.1));
			doc.push(elements::PageBreak::new());
			doc.push(note_img.clone());
			doc.push(
				elements::Paragraph::new(&self.name)
					.aligned(Alignment::Right)
					.styled(Style::new().with_font_size(17))
					.padded(Margins::trbl(29, 13, 0, 0)),
			);
			doc.push(
				elements::Paragraph::new(&self.subject)
					.aligned(Alignment::Right)
					.styled(Style::new().with_font_size(17))
					.padded(Margins::trbl(0, 13, 0, 0)),
			);
			Ok(())
		})
	}
}
