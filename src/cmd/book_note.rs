use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use crate::cli::DIALOGUER_THEME;
use crate::options::{Book, Options};

use super::note;

pub fn cli_run() {
	let options = Options::read().unwrap();
	let theme: &ColorfulTheme = &DIALOGUER_THEME;
	let book: &Book = options
		.books
		.get(
			Select::with_theme(theme)
				.with_prompt("Book")
				.items(options.books.as_slice())
				.interact()
				.expect("Failed to ask user for book"),
		)
		.unwrap();
	let chapter: String = Input::with_theme(theme)
		.with_prompt("Chapter")
		.interact_text()
		.expect("Failed to ask user for chapter name");
	note::new(
		&chapter,
		&format!("{} by {}", &book.name, &book.author),
		&PathBuf::from("Books")
			.join(&book.name)
			.to_str()
			.unwrap()
			.to_string(),
		2,
	)
	.unwrap();
}
