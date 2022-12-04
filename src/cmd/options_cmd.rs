use clap::ArgMatches;

use crate::options::Options;

pub fn cli_run(args: &ArgMatches) {
	let options = Options::read().expect("Failed to read options");
	let option: &String = args.get_one("option").unwrap();
	match option.as_str() {
		"class.names" => {
			for class in options.classes {
				println!("{}", class.name);
			}
		}
		_ => unreachable!(),
	}
}
