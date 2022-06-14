use std::{fs::File, io::BufWriter};

mod write;

use printpdf::*;
use task_log::task;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("OpenAPI", Mm(595.0), Mm(842.0), "black");
    let page = doc.get_page(page1);
    let font = doc
        .add_external_font(
            File::open("GainsboroughSans-Regular.otf").expect("Failed to read font file"),
        )
        .expect("Failed to read font");

    let black_layer = page.get_layer(layer1);
    let white_layer = page.add_layer("white");
    white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

    task("Writing name", || {
        write::name(&black_layer, &white_layer, &font);
    });
    task("Writing title", || {
        write::title(&black_layer, &font, "Note");
    });
    task("Writing lines", || {
        write::lines(&black_layer, true);
    });
    task("Writing logo", || {
        write::logo(&black_layer).expect("Failed to write logo");
    });

    task("Saving PDF", || {
        doc.save(&mut BufWriter::new(
            File::create("main.pdf").expect("Failed to create main.pdf file"),
        ))
        .expect("Failed to save pdf");
    });
}
