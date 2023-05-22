use std::env;

use cmd::{book_note, food_log, options_cmd, school_note};
use options::Options;

mod cli;
mod cmd;
mod document;
mod options;

fn main() {
	env::set_current_dir("/Users/matt/src/neptune").expect("Failed to change path to repo path");
	let options = Options::read().expect("Failed to parse options");
	let matches = cli::setup(&options).get_matches();
	match matches.subcommand() {
		Some(("school-note", args)) => school_note::cli_run(args),
		Some(("book-note", _)) => book_note::cli_run(),
		Some(("food-log", _)) => food_log::cli_run(),
		Some(("options", args)) => options_cmd::cli_run(args),
		_ => unreachable!(),
	}
}
