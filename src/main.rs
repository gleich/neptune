use std::{fs::File, io::BufWriter};

mod write;

use printpdf::*;
use task_log::task;

const WIDTH: f64 = 595.0;
const HEIGHT: f64 = 842.0;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("OpenAPI", Mm(WIDTH), Mm(HEIGHT), "black");
    let mut page = doc.get_page(page1);
    let font = doc
        .add_external_font(
            File::open("GainsboroughSans-Regular.otf").expect("Failed to read font file"),
        )
        .expect("Failed to read font");

    for i in 1..10 {
        let black_layer = page.get_layer(layer1);
        let white_layer = page.add_layer("white");
        white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

        task(format!("Generating page {}", i), || {
            write::name(&black_layer, &white_layer, &font);
            write::title(&black_layer, &font, "Note");
            write::lines(&black_layer, true);
            write::logo(&black_layer).expect("Failed to write logo");
            write::page_number(i, &black_layer, &font);
        });
        let (new_page, _) = doc.add_page(Mm(WIDTH), Mm(HEIGHT), "black");
        page = doc.get_page(new_page);
    }

    task("Saving PDF", || {
        doc.save(&mut BufWriter::new(
            File::create("main.pdf").expect("Failed to create main.pdf file"),
        ))
        .expect("Failed to save pdf");
    });
}
