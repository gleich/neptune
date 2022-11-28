use std::env;

use cmd::{book_note, school_note};

mod cli;
mod cmd;
mod document;
mod options;

fn main() {
	env::set_current_dir("/Users/matt/src/neptune").expect("Failed to change path to repo path");
	let matches = cli::setup().get_matches();
	match matches.subcommand() {
		Some(("school-note", _)) => school_note::cli_run(),
		Some(("book-note", _)) => book_note::cli_run(),
		_ => unreachable!(),
	}
}
