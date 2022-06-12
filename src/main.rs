use std::{fs::File, io::BufWriter};

use chrono::Local;
use printpdf::{Color, IndirectFontRef, Line, Mm, PdfDocument, PdfLayerReference, Point, Rgb};

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Daily Log #3", Mm(595.0), Mm(842.0), "black");
    let black_layer = doc.get_page(page1).get_layer(layer1);
    let white_layer = doc.get_page(page1).add_layer("white");
    let font = doc
        .add_external_font(
            File::open("./GainsboroughSans-Regular.otf").expect("Failed to read from font file"),
        )
        .expect("Failed to read font");
    white_layer.set_fill_color(Color::Rgb(Rgb::new(255.0, 255.0, 255.0, None)));

    write_name(&black_layer, &white_layer, &font);
    write_title(&black_layer, &font, "Daily Log");

    doc.save(&mut BufWriter::new(
        File::create("main.pdf").expect("Failed to create main.pdf file"),
    ))
    .expect("Failed to save pdf");
}

const NAME: &str = "Matt Gleich";
fn write_name(
    black_layer: &PdfLayerReference,
    white_layer: &PdfLayerReference,
    font: &IndirectFontRef,
) {
    white_layer.use_text(NAME, 80.0, Mm(10.0), Mm(815.0), &font);

    let points1 = vec![
        (Point::new(Mm(6.0), Mm(810.0)), false),
        (Point::new(Mm(6.0), Mm(835.0)), false),
        (Point::new(Mm(133.0), Mm(835.0)), false),
        (Point::new(Mm(133.0), Mm(810.0)), false),
    ];
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    black_layer.set_outline_thickness(0.0);
    black_layer.add_shape(line1);
}

fn write_title(black_layer: &PdfLayerReference, font: &IndirectFontRef, title: &str) {
    black_layer.use_text(title, 45.0, Mm(140.0), Mm(825.0), &font);
    let now = Local::now();
    black_layer.use_text(
        now.format("%m.%d.%y - %A").to_string(),
        45.0,
        Mm(140.0),
        Mm(811.0),
        &font,
    );
}
