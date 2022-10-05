use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use crate::cli::DIALOGUER_THEME;
use crate::cmd::note;
use crate::options::{Class, Options};

pub fn cli_run() {
	let options = Options::read().unwrap();
	let theme: &ColorfulTheme = &*DIALOGUER_THEME;
	let name: String = Input::with_theme(theme)
		.with_prompt("Name")
		.interact_text()
		.expect("Failed to ask user for document name");
	let class: &Class = options
		.classes
		.get(
			Select::with_theme(theme)
				.with_prompt("Class")
				.items(options.classes.as_slice())
				.interact()
				.expect("Failed to ask user for class"),
		)
		.unwrap();
	let folder: String = Input::with_theme(theme)
		.with_prompt("Folder")
		.allow_empty(true)
		.interact_text()
		.expect("Failed to ask user for folder name");
	note::new(
		name,
		&class.to_string(),
		PathBuf::from("College")
			.join("Notes")
			.join(&class.id)
			.join(folder)
			.to_str()
			.unwrap()
			.to_string(),
	)
	.unwrap();
}
