use chrono::Local;
use printpdf::Mm;

use crate::{
    auth::NeptuneToken,
    document::{Document, HEIGHT, WIDTH},
    result::Result,
    write,
};

#[post("/note?<folder>&<name>")]
pub fn route(_token: NeptuneToken, folder: String, name: String) -> Result<String> {
    let now = Local::now();
    let document = Document::new(&name).expect("Failed to create document");
    write::lines(&document.black_layer, true, 24);
    write::name(&document, Mm(15.0), Mm(800.0));
    write::logo(&document, Mm(480.0), Mm(750.0), 0.50).expect("Failed to write logo to document");
    document
        .black_layer
        .use_text("Note", 50.0, Mm(15.0), Mm(780.0), &document.title_font);
    document
        .black_layer
        .use_text(&folder, 50.0, Mm(15.0), Mm(765.0), &document.title_font);
    document
        .black_layer
        .use_text(&name, 50.0, Mm(15.0), Mm(750.0), &document.title_font);
    document.black_layer.use_text(
        now.format("%D").to_string(),
        50.0,
        Mm(15.0),
        Mm(735.0),
        &document.title_font,
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
    document
        .upload(format!("/Notes/{}", folder))
        .expect("Failed to upload document");
    Ok(format!("Created {} in {}", name, folder))
}
