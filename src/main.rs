use std::{fs::File, io::BufWriter};

mod cli;
mod document;

use printpdf::*;
use task_log::task;

fn main() {
    let matches = cli::setup().get_matches();
    match matches.subcommand() {
        Some(("note", _)) => {}
        _ => unreachable!(),
    }
}
