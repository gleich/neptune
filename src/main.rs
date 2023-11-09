use templates::note::Note;

mod document;
mod endpoints;
mod goodnotes;
mod templates;

fn main() {
	let note = Note {
		name: String::from("TESTING TESTING"),
		folder: String::from("TESTING TESTING"),
		subject: String::from("CSCI 240"),
	};
	let document = note.create().expect("Failed to create note");
	document::save(&note.name, document).expect("Failed to save document");
}
