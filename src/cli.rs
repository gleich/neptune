use clap::Command;

pub fn setup() -> Command<'static> {
	Command::new("neptune")
		.version("3.0.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("reMarkable PDF automated templates")
		.arg_required_else_help(true)
		.subcommand(Command::new("school-note").about("Create a school note"))
}
