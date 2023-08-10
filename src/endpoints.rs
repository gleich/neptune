use std::path::Path;

use anyhow::Context;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;

use crate::{rmapi, templates};

#[post("/note", data = "<note>")]
pub fn note(note: Json<templates::note::Note>) -> Result<String> {
	let doc = note.create().context("Failed to create document")?;
	rmapi::upload(
		doc,
		&note.name,
		&Path::new("College").join(&note.subject).join(&note.folder),
	)
	.context("Failed to upload document")?;
	Ok(String::from("Created and uploaded note!"))
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
