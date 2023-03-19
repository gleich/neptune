use clap::{Arg, Command};
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;

use crate::options::Options;

lazy_static! {
	pub static ref DIALOGUER_THEME: ColorfulTheme = ColorfulTheme::default();
}

pub fn setup(options: &Options) -> Command {
	Command::new("neptune")
		.version("3.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("reMarkable PDF automated templates")
		.arg_required_else_help(true)
		.subcommand(
			Command::new("school-note")
				.about("Create a school note")
				.arg(
					Arg::new("class")
						.short('c')
						.long("class")
						.value_parser(
							options
								.classes
								.iter()
								.cloned()
								.map(|c| c.name)
								.collect::<Vec<String>>(),
						)
						.required(false)
						.help("Name of the class"),
				)
				.arg(
					Arg::new("name")
						.long("name")
						.short('n')
						.help("Name of the document"),
				)
				.arg(
					Arg::new("folder")
						.long("folder")
						.short('f')
						.help("Name of the folder"),
				)
				.arg(
					Arg::new("category")
						.long("category")
						.short('t')
						.help("Category of the document")
						.value_parser(["Notes", "Practice", "Assessment"]),
				),
		)
		.subcommand(
			Command::new("book-note")
				.about("Create a book/article note")
				.alias("article-note"),
		)
		.subcommand(
			Command::new("options")
				.about("Get values from the configuration file")
				.hide(true)
				.arg(
					Arg::new("option")
						.required(true)
						.value_parser(["class.names"]),
				),
		)
}
