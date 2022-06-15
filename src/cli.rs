use clap::Command;

pub fn setup() -> Command<'static> {
    Command::new("neptune")
        .version("1.0.0")
        .author("Matt Gleich <email@mattglei.ch>")
        .about("PDF document generator")
        .arg_required_else_help(true)
        .subcommand(Command::new("daily-log").about("Generate daily log PDF"))
}
