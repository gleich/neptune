use cmd::school_note::cli_run;

mod cli;
mod cmd;
mod document;

fn main() {
	let matches = cli::setup().get_matches();
	match matches.subcommand() {
		Some(("school-note", _)) => cli_run(),
		_ => unreachable!(),
	}
}
