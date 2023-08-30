use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::ArgMatches;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};

use crate::cli::DIALOGUER_THEME;
use crate::cmd::note;
use crate::options::{Class, Options};

pub const CATEGORIES: [&str; 4] = ["Note", "Practice", "Worksheet", "Other"];

#[derive(Debug)]
pub struct Inputs {
	pub name: String,
	pub class: Class,
	pub folder: String,
	pub category: String,
	pub pages: u32,
}

pub fn cli_run(args: &ArgMatches) {
	let options = Options::read().expect("Failed to read options");

	let inputs = ask(args, &options).expect("Failed to ask for inputs");
	note::new(
		inputs.name,
		&inputs.class.to_string(),
		PathBuf::from("College")
			.join(inputs.class.id)
			.join(inputs.category)
			.join(inputs.folder)
			.to_str()
			.unwrap()
			.to_string(),
		inputs.pages,
	)
	.unwrap();
}

fn ask(args: &ArgMatches, options: &Options) -> Result<Inputs> {
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

	// getting the category
	let category_arg: Option<&String> = args.get_one("category");
	let category = match category_arg {
		Some(x) => x.to_owned(),
		None => CATEGORIES
			.get(
				FuzzySelect::with_theme(theme)
					.with_prompt("Category")
					.items(&CATEGORIES)
					.interact()
					.context("asking for category failed")?,
			)
			.unwrap()
			.to_string(),
	};

	// getting the class
	let class_arg: Option<&String> = args.get_one("class");
	let class = match class_arg {
		Some(x) => options.classes.iter().find(|c| &c.name == x),
		None => options.classes.get(
			FuzzySelect::with_theme(theme)
				.with_prompt("Class")
				.items(options.classes.as_slice())
				.interact()
				.context("Failed to ask user for class")?,
		),
	};

	// getting the folder name
	let folder_arg: Option<&String> = args.get_one("folder");
	let folder = match folder_arg {
		Some(x) => x.to_owned(),
		None => Input::with_theme(theme)
			.with_prompt("Folder")
			.allow_empty(true)
			.interact_text()
			.context("Failed to ask user for folder name")?,
	};

	// getting the pages
	let pages_arg: Option<&String> = args.get_one("name");
	let pages = match pages_arg {
		Some(x) => x.to_owned(),
		None => {
			let input_name: String = Input::with_theme(theme)
				.with_prompt("Pages")
				.default(String::from("1"))
				.interact_text()
				.context("Failed to ask user for number of pages")?;
			input_name
		}
	};

	Ok(Inputs {
		name,
		class: class.unwrap().to_owned(),
		folder,
		category,
		pages: pages.parse().unwrap(),
	})
}
