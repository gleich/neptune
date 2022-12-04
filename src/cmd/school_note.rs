use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::ArgMatches;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use crate::cli::DIALOGUER_THEME;
use crate::cmd::note;
use crate::options::{Class, Options};

pub fn cli_run(args: &ArgMatches) {
	let options = Options::read().expect("Failed to read options");

	let (name, class, folder) = ask(args, &options).expect("Failed to ask for inputs");
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

fn ask<'a>(args: &ArgMatches, options: &'a Options) -> Result<(String, &'a Class, String)> {
	let theme: &ColorfulTheme = &DIALOGUER_THEME;

	// getting name of document
	let name_arg: Option<&String> = args.get_one("name");
	let name = match name_arg {
		Some(x) => x.to_owned(),
		None => {
			let input_name: String = Input::with_theme(theme)
				.with_prompt("Name")
				.interact_text()
				.context("Failed to ask user for document name")?;
			input_name
		}
	};

	// getting the class
	let class_arg: Option<&String> = args.get_one("class");
	let class = match class_arg {
		Some(x) => options.classes.iter().find(|c| &c.name == x),
		None => {
			options.classes.get(
				Select::with_theme(theme)
					.with_prompt("Class")
					.items(options.classes.as_slice())
					.interact()
					.context("Failed to ask user for class")?,
			)
		}
	};

	// getting the folder name
	let folder_arg: Option<&String> = args.get_one("folder");
	let folder = match folder_arg {
		Some(x) => x.to_owned(),
		None => {
			Input::with_theme(theme)
				.with_prompt("Folder")
				.allow_empty(true)
				.interact_text()
				.context("Failed to ask user for folder name")?
		}
	};

	Ok((name, class.unwrap(), folder))
}
