use clap::Command;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref DIALOGUER_THEME: ColorfulTheme = ColorfulTheme::default();
}

pub fn setup() -> Command<'static> {
	Command::new("neptune")
		.version("3.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("reMarkable PDF automated templates")
		.arg_required_else_help(true)
		.subcommand(Command::new("school-note").about("Create a school note"))
		.subcommand(
			Command::new("book-note")
				.about("Create a book/article note")
				.alias("article-note"),
		)
}
