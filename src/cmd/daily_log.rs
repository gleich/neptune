use anyhow::Context;
use chrono::{Datelike, Local};
use ordinal::Ordinal;
use printpdf::*;

use crate::{
    auth::NeptuneToken,
    document::{Document, HEIGHT, WIDTH},
    result::Result,
    write,
};

#[post("/daily-log")]
pub fn route(_token: NeptuneToken) -> Result<String> {
    let now = Local::now();
    let name = &now
        .format(&format!("%d{} (%A)", Ordinal(now.day()).suffix()))
        .to_string();

    let document = Document::new(name).context("Failed to create new document")?;

    // generating document
    write::giant_date(&document, Mm(45.0), Mm(640.0), now);
    write::days(&document, Mm(45.0), Mm(610.0), now);
    write::logo(&document, Mm(390.0), Mm(640.0), 0.95).context("Failed to write logo")?;
    write::lines(&document.black_layer, false, 19);
    let (second_page_index, layer1) = document.document.add_page(WIDTH, HEIGHT, "black");
    write::lines(
        &document
            .document
            .get_page(second_page_index)
            .get_layer(layer1),
        false,
        26,
    );
    write::name(&document, Mm(405.0), Mm(625.0));
    write::date_information(&document, Mm(225.0), Mm(675.0), now);

    // uploading document
    document
        .upload(now.format("/Daily Logs/%B/").to_string())
        .context("Failed to upload document")?;
    Ok(format!("Created daily log: {}", name))
}
