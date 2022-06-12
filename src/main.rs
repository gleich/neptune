use std::{fs::File, io::BufWriter};

use printpdf::{Mm, PdfDocument};

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Daily Log #3", Mm(595.0), Mm(842.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let text = "Testing Testing";
    let font = doc
        .add_external_font(
            File::open("./GainsboroughSans-Regular.otf").expect("Failed to read from font file"),
        )
        .expect("Failed to read font");
    current_layer.use_text(text, 30.0, Mm(200.0), Mm(200.0), &font);

    doc.save(&mut BufWriter::new(
        File::create("main.pdf").expect("Failed to create main.pdf file"),
    ))
    .expect("Failed to save pdf");
}
