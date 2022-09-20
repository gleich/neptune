use document::Document;

mod document;
mod resources;

fn main() {
	let document =
		Document::new::<&str, &str>("Testing", "Foo").expect("Failed to create document");
	document.debug_save().expect("Failed to save document");
}
