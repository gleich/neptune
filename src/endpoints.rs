// pub async fn note(note: Json<templates::note::Note>) -> EventStream![] {
// 	let doc = note
// 		.create()
// 		.context("Failed to create document")
// 		.expect("failed to create note");
// 	EventStream! {
// 		let note_cloned = note.clone();
// 		yield Event::data("created document");
// 		yield Event::data("uploading document");
// 		rmapi::upload(
// 			doc,
// 			&note_cloned.name,
// 			&Path::new("College").join(&note_cloned.subject).join(&note_cloned.folder),
// 		);
// 	}
// }
