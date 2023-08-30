use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use rocket::response::stream::Event;
use rocket::response::stream::EventStream;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast::Sender;
use rocket::{serde, Shutdown, State};

use crate::{rmapi, templates};

#[post("/note", data = "<note>")]
pub async fn note(note: Json<templates::note::Note>) -> EventStream![] {
	let doc = note
		.create()
		.context("Failed to create document")
		.expect("failed to create note");
	EventStream! {
		let note_cloned = note.clone();
		yield Event::data("created document");
		yield Event::data("uploading document");
		rmapi::upload(
			doc,
			&note_cloned.name,
			&Path::new("College").join(&note_cloned.subject).join(&note_cloned.folder),
		);
	}
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
	pub message: String,
	pub progress: u32,
}

#[derive(Debug)]
pub struct Error(anyhow::Error);
pub type Result<T = ()> = std::result::Result<T, Error>;

impl<E> From<E> for Error
where
	E: Into<anyhow::Error>,
{
	fn from(error: E) -> Self {
		Error(error.into())
	}
}

impl<'r> Responder<'r, 'static> for Error {
	fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		response::Debug(self.0).respond_to(request)
	}
}
