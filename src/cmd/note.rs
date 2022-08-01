use chrono::Local;
use printpdf::Mm;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

use crate::{
    auth::NeptuneToken,
    document::{Document, HEIGHT, WIDTH},
    result::Result,
    write,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InputData {
    pub name: String,
    pub folder: String,
}

#[post("/note", data = "<inputs>")]
pub fn route(_token: NeptuneToken, inputs: Json<InputData>) -> Result<String> {
    let now = Local::now();
    let document = Document::new(&inputs.name).expect("Failed to create document");
    write::lines(&document.layers.black, true, 24);
    write::name(&document, Mm(15.0), Mm(800.0));
    write::logo(&document, Mm(480.0), Mm(750.0), 0.50).expect("Failed to write logo to document");
    document.layers.black.use_text(
        "Note",
        50.0,
        Mm(15.0),
        Mm(780.0),
        &document.fonts.gainsborough_sans_regular,
    );
    document.layers.black.use_text(
        &inputs.folder,
        50.0,
        Mm(15.0),
        Mm(765.0),
        &document.fonts.gainsborough_sans_regular,
    );
    document.layers.black.use_text(
        &inputs.name,
        50.0,
        Mm(15.0),
        Mm(750.0),
        &document.fonts.gainsborough_sans_regular,
    );
    document.layers.black.use_text(
        now.format("%D").to_string(),
        50.0,
        Mm(15.0),
        Mm(735.0),
        &document.fonts.gainsborough_sans_regular,
    );
    let (second_page_index, layer1) = document.document.add_page(WIDTH, HEIGHT, "black");
    write::lines(
        &document
            .document
            .get_page(second_page_index)
            .get_layer(layer1),
        true,
        25,
    );
    let full_path = format!("/Notes/{}", &inputs.folder);
    document
        .upload(&full_path)
        .expect("Failed to upload document");
    Ok(format!("Created {} in {}", inputs.name, full_path))
}
