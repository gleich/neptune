use templates::note::Note;

mod document;
mod endpoints;
mod templates;

fn main() {
	let note = Note {
		name: String::from("TESTING TESTING"),
		folder: String::from("TESTING TESTING"),
		subject: String::from("CSCI 240"),
	};
	let document = note.create().expect("Failed to create note");
	let saved_path = document::save(&note.name, document).expect("Failed to save document");
	document::open(&saved_path).expect("Failed to open document");
}
