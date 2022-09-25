use cmd::school_note::{raw_run, Class};

mod cmd;
mod document;

fn main() {
	raw_run("Introduction to Limits", Class::MATH171, "Testing")
		.expect("Failed to generate a raw run");
}
