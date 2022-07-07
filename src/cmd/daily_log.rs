use chrono::{Datelike, Local};
use ordinal::Ordinal;
use printpdf::*;
use task_log::task;

use crate::{
    document::{Document, HEIGHT, WIDTH},
    write,
};

pub fn run() {
    let now = Local::now();

    let document = Document::new(
        &now.format(&format!("%d{} (%A)", Ordinal(now.day()).suffix()))
            .to_string(),
    )
    .expect("Failed to create new document");
    task("Generating document", || {
        write::giant_date(&document, Mm(45.0), Mm(640.0), now);
        write::days(&document, Mm(45.0), Mm(610.0), now);
        write::logo(&document, Mm(390.0), Mm(640.0), 0.95).expect("Failed to write logo");
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
    });

    task("Uploading document", || {
        document
            .upload(now.format("/Daily Logs/%B/").to_string())
            .expect("Failed to upload document");
    });
}
