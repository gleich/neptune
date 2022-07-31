use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
    process::Command,
};

use anyhow::{ensure, Context, Result};
use printpdf::{
    Color, Greyscale, IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference,
    PdfPageIndex, PdfPageReference,
};

pub struct Document {
    pub name: String,
    pub filename: String,
    pub document: PdfDocumentReference,
    pub first_page_index: PdfPageIndex,
    pub first_page_reference: PdfPageReference,
    pub fonts: Fonts,
    pub layers: Layers,
}

pub struct Fonts {
    pub computer_modern_italic: IndirectFontRef,
    pub gainsborough_sans_regular: IndirectFontRef,
}

pub struct Layers {
    pub black: PdfLayerReference,
    pub white: PdfLayerReference,
}

pub const WIDTH: Mm = Mm(595.0);
pub const HEIGHT: Mm = Mm(842.0);

impl Document {
    pub fn new(name: &str) -> Result<Self> {
        let (doc, first_page_index, layer1) = PdfDocument::new(name, WIDTH, HEIGHT, "black");
        let first_page_reference = doc.get_page(first_page_index);
        let black_layer = first_page_reference.get_layer(layer1);
        let white_layer = first_page_reference.add_layer("white");

        black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
        white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

        let assets_folder = Path::new("assets");

        Ok(Self {
            name: name.to_string(),
            filename: format!("{}.pdf", name),
            first_page_index,
            first_page_reference,
            fonts: Fonts {
                computer_modern_italic: doc.add_external_font(File::open(
                    assets_folder.join("Computer Modern").join("italic.ttf"),
                )?)?,
                gainsborough_sans_regular: doc.add_external_font(File::open(
                    assets_folder.join("Gainsborough Sans").join("regular.otf"),
                )?)?,
            },
            layers: Layers {
                black: black_layer,
                white: white_layer,
            },
            document: doc,
        })
    }

    pub fn upload(self, folder: String) -> Result<()> {
        self.document
            .save(&mut BufWriter::new(
                File::create(&self.filename)
                    .expect(&format!("Failed to create {}", &self.filename)),
            ))
            .context("Failed to save document")?;

        let mut process = Command::new("rmapi")
            .arg("mkdir")
            .arg(&folder)
            .spawn()
            .context("Failed to spawn process to make parent directory")?;
        process.stdout.take();
        let mut status = process.wait().context("Failed to make parent directory")?;
        ensure!(status.success());

        process = Command::new("rmapi")
            .arg("put")
            .arg(&self.filename)
            .arg(&folder)
            .spawn()
            .context("Failed to spawn process to upload document")?;
        process.stdout.take();
        status = process.wait().context("Failed to make parent directory")?;
        ensure!(status.success());

        fs::remove_file(self.filename)
            .context("Failed to remove file after upload")
            .context("Failed to delete pdf")?;

        Ok(())
    }
}
