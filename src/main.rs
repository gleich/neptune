use std::{fs::File, io::BufWriter};

use anyhow::{Context, Result};
use chrono::Local;
use printpdf::*;
use task_log::task;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("OpenAPI", Mm(595.0), Mm(842.0), "black");
    let black_layer = doc.get_page(page1).get_layer(layer1);
    let white_layer = doc.get_page(page1).add_layer("white");
    let font = doc
        .add_external_font(
            File::open("GainsboroughSans-Regular.otf").expect("Failed to read font file"),
        )
        .expect("Failed to read font");
    white_layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None)));

    task("Writing name", || {
        write_name(&black_layer, &white_layer, &font);
    });
    task("Writing title", || {
        write_title(&black_layer, &font, "Note");
    });
    task("Writing lines", || {
        write_lines(&black_layer, true);
    });
    task("Writing logo", || {
        write_logo(black_layer).expect("Failed to write logo");
    });
    task("Saving PDF", || {
        doc.save(&mut BufWriter::new(
            File::create("main.pdf").expect("Failed to create main.pdf file"),
        ))
        .expect("Failed to save pdf");
    });
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

fn write_logo(black_layer: PdfLayerReference) -> Result<()> {
    let mut image = File::open("logo.jpg").context("Failed to read logo.png file")?;
    let image = Image::try_from(
        image_crate::codecs::jpeg::JpegDecoder::new(&mut image)
            .context("Failed to decode png logo")?,
    )
    .context("Failed to convert codecs to Image")?;
    image.add_to_layer(
        black_layer,
        ImageTransform {
            translate_x: Some(Mm(494.0)),
            translate_y: None,
            rotate: None,
            scale_x: Some(0.6),
            scale_y: Some(0.6),
            dpi: None,
        },
    );
    Ok(())
}

fn write_lines(black_layer: &PdfLayerReference, cornell_style: bool) {
    let (mut x1, mut x2) = (if cornell_style { 200.0 } else { 100.0 }, 495.0);
    let spacing = 30;
    let width = 1.0;
    let lines = 22;
    let bottom_margin = 110;
    if cornell_style {
        let (note_x1, note_x2) = (160.0, 170.0);
        let (note_y1, note_y2) = (
            (1 * spacing + bottom_margin) as f64 - 20.0,
            (lines * spacing + bottom_margin) as f64,
        );
        let line = Line {
            points: vec![
                (Point::new(Mm(note_x1), Mm(note_y1)), false),
                (Point::new(Mm(note_x2), Mm(note_y1)), false),
                (Point::new(Mm(note_x2), Mm(note_y2)), false),
                (Point::new(Mm(note_x1), Mm(note_y2)), false),
            ],
            is_closed: false,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false,
        };
        black_layer.add_shape(line);
    }
    black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.40, None)));
    for i in 1..lines {
        let y = (i * spacing + bottom_margin) as f64;
        if i == lines - 1 && cornell_style {
            x1 = 350.0;
            x2 = 495.0;
        }
        let line = Line {
            points: vec![
                (Point::new(Mm(x1), Mm(y)), false),
                (Point::new(Mm(x2), Mm(y)), false),
                (Point::new(Mm(x2), Mm(y + width)), false),
                (Point::new(Mm(x1), Mm(y + width)), false),
            ],
            is_closed: false,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };
        black_layer.add_shape(line);
    }
    black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
}
