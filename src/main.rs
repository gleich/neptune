use document::Document;
use task_log::task;

mod document;

fn main() {
	let document = task("Creating document", || {
		Document::new("hello world").expect("Failed to create document")
	});
	document
		.debug_save()
		.expect("Failed to debug save document");
}
