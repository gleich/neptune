use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};
use templates::note::Note;

mod document;
mod templates;

fn main() {
	let note = ask().expect("Failed to ask user questions");
	let document = note.create().expect("Failed to create note");
	let saved_path = document::save(&note.name, document).expect("Failed to save document");
	document::open(&saved_path).expect("Failed to open document");
}

fn ask() -> Result<Note> {
	let theme: &ColorfulTheme = &ColorfulTheme::default();

	let name: String = Input::with_theme(theme)
		.with_prompt("Name")
		.interact_text()
		.context("Failed to ask user for document name")?;

	let subjects = [
		"University Physics 1A [PHYS 211A]",
		"First Year Writing: Writing Seminar [UWRT 150]",
		"The Mechanics of Programming [CSCI 243]",
	];
	let subject = subjects
		.get(
			FuzzySelect::with_theme(theme)
				.with_prompt("Subject")
				.items(&subjects)
				.interact()
				.context("asking for subject failed")?,
		)
		.unwrap();

	println!();
	Ok(Note {
		name,
		subject: subject.to_string(),
	})
}
