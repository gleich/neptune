use anyhow::{Context, Result};
use chrono::Local;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use printpdf::Mm;
use task_log::task;

use crate::{
    document::{Document, HEIGHT, WIDTH},
    write,
};

pub fn run() {
    let (name, folder) = ask().expect("Failed to ask user questions");

    let now = Local::now();
    let document = Document::new(&name).expect("Failed to create document");
    task("Generating document", || {
        write::lines(&document.black_layer, true, 24);
        write::name(&document, Mm(15.0), Mm(800.0));
        write::logo(&document, Mm(480.0), Mm(750.0), 0.50)
            .expect("Failed to write logo to document");
        document
            .black_layer
            .use_text("Note", 50.0, Mm(15.0), Mm(780.0), &document.title_font);
        document
            .black_layer
            .use_text(&folder, 50.0, Mm(15.0), Mm(765.0), &document.title_font);
        document
            .black_layer
            .use_text(name, 50.0, Mm(15.0), Mm(750.0), &document.title_font);
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
    });
    task("Uploading document", || {
        document
            .upload(format!("/Notes/{}", folder))
            .expect("Failed to upload document");
    });
}

pub fn ask() -> Result<(String, String)> {
    let theme = ColorfulTheme::default();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Name")
        .interact_text()
        .context("Failed to ask user for name of document")?;

    let folders = ["Math", "Stainless", "Coding"];
    let folder = folders[FuzzySelect::with_theme(&theme)
        .with_prompt("Folder")
        .items(&folders)
        .default(0)
        .interact()
        .context("Failed to ask user for folder")?];

    Ok((name, folder.to_string()))
}
