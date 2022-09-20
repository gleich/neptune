use document::Document;

mod document;
mod resources;

fn main() {
	Document::new::<&str, &str>("Testing", "Foo").expect("Failed to create document");
	println!("Testing");
}
