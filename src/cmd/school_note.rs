use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use strum::VariantNames;
use strum_macros::{EnumVariantNames, FromRepr};

use crate::cli::DIALOGUER_THEME;
use crate::cmd::note;

pub fn cli_run() {
	let theme: &ColorfulTheme = &*DIALOGUER_THEME;
	let name: String = Input::with_theme(theme)
		.with_prompt("Name")
		.interact_text()
		.expect("Failed to ask user for document name");
	let class = Class::from_repr(
		Select::with_theme(theme)
			.with_prompt("Class")
			.items(Class::VARIANTS)
			.default(0)
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
			.join(class.short_name())
			.join(folder)
			.to_str()
			.unwrap()
			.to_string(),
	)
	.unwrap();
}

#[derive(EnumVariantNames, FromRepr)]
pub enum Class {
	MATH171,
	MEDG101,
	PHIL101,
}

impl ToString for Class {
	fn to_string(&self) -> String {
		match self {
			Self::MATH171 => format!("{}: Calculus A", self.short_name()),
			Self::MEDG101 => format!("{}: Human Biology 1", self.short_name()),
			Self::PHIL101 => format!("{}: Intro to Philosophy", self.short_name()),
		}
	}
}

impl Class {
	pub fn short_name(&self) -> String {
		match self {
			Self::MATH171 => String::from("MATH 171"),
			Self::MEDG101 => String::from("MEDG 110"),
			Self::PHIL101 => String::from("PHIL 101"),
		}
	}
}
