use std::{fs::File, io::BufWriter};

use anyhow::{Context, Result};
use printpdf::{
    Color, Greyscale, IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference,
    PdfPageIndex, PdfPageReference,
};

pub struct Document {
    pub name: String,
    pub document: PdfDocumentReference,
    pub first_page_index: PdfPageIndex,
    pub first_page_reference: PdfPageReference,
    pub title_font: IndirectFontRef,
    pub giant_font: IndirectFontRef,
    pub black_layer: PdfLayerReference,
    pub white_layer: PdfLayerReference,
}

impl Document {
    pub fn new(name: &str) -> Result<Self> {
        let (doc, first_page_index, layer1) = PdfDocument::new(name, Mm(595.0), Mm(842.0), "black");
        let first_page_reference = doc.get_page(first_page_index);
        let title_font = doc
            .add_external_font(
                File::open("GainsboroughSans-Regular.otf").context("Failed to read font file")?,
            )
            .context("Failed to read font")?;
        let giant_font = doc
            .add_external_font(File::open("cmunti.ttf").context("Failed to read font file")?)
            .context("Failed to read font")?;
        let black_layer = first_page_reference.get_layer(layer1);
        let white_layer = first_page_reference.add_layer("white");

        black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
        white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

        Ok(Self {
            name: name.to_string(),
            document: doc,
            first_page_index,
            first_page_reference,
            title_font,
            giant_font,
            black_layer,
            white_layer,
        })
    }

    pub fn save(self) -> Result<()> {
        let filename = format!("{}.pdf", self.name);
        self.document
            .save(&mut BufWriter::new(
                File::create(&filename).expect(&format!("Failed to create {}", &filename)),
            ))
            .context("Failed to save document")?;
        Ok(())
    }
}
