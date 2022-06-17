mod cli;
mod cmd;
mod document;
mod write;

fn main() {
    let matches = cli::setup().get_matches();
    match matches.subcommand() {
        Some(("daily-log", _)) => cmd::daily_log::run(),
        Some(("note", _)) => cmd::note::run(),
        _ => unreachable!(),
    }
}
