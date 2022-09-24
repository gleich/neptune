use document::Document;
use task_log::task;

mod cmd;
mod document;
mod resources;
mod writer;

fn main() {
	let document = task("Setting up document", || -> Document {
		Document::new::<&str, &str>("Testing", "Foo").expect("Failed to create document")
	});
	document.draw_rectangle(
		resources::WIDTH - 10.0,
		resources::HEIGHT - 10.0,
		10,
		10,
		true,
		true,
	);
	document.draw_rectangle(0, 0, 10, 10, true, true);
	document
		.draw_logo(135, 300, 1)
		.expect("Failed to draw logo");
	task("Saving document", || {
		document.debug_save().expect("Failed to save document");
	});
}
