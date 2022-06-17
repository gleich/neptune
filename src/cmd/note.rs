use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::document::Document;

pub fn run() {
    let (name, folder) = ask().expect("Failed to ask user information about documents");
}

pub fn ask() -> Result<(String, String)> {
    let theme = ColorfulTheme::default();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Name")
        .interact_text()
        .context("Failed to ask user for name of document")?;

    let folders = ["Math"];
    let folder = folders[FuzzySelect::with_theme(&theme)
        .with_prompt("Folder")
        .items(&folders)
        .default(0)
        .interact()
        .context("Failed to ask user for folder")?];

    Ok((name, folder.to_string()))
}
