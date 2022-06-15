use std::{fs::File, thread, time::Duration};

use anyhow::{Context, Result};
use chrono::Local;
use printpdf::*;

use crate::{document::Document, write};

pub fn run() {
    let document = Document::new("Name").expect("Failed to create new document");
    write::giant_date(&document, Mm(45.0), Mm(640.0));
    write::days(&document, Mm(45.0), Mm(610.0));
    write::logo(&document, Mm(390.0), Mm(640.0), 0.95);
    lines(&document.black_layer, false, 19);
    document.save().expect("Failed to save document");
}

pub fn title(black_layer: &PdfLayerReference, font: &IndirectFontRef, title: &str) {
    black_layer.begin_text_section();
    black_layer.end_text_section();
    let x = 330.0;
    black_layer.use_text(title, 45.0, Mm(x), Mm(825.0), &font);
    let now = Local::now();
    black_layer.use_text(
        now.format("%m.%d.%y - %A").to_string(),
        45.0,
        Mm(x),
        Mm(811.0),
        &font,
    );
}

pub fn lines(black_layer: &PdfLayerReference, cornell_style: bool, lines: usize) {
    let (mut x1, x2) = (
        if cornell_style { 200.0 } else { 100.0 },
        if cornell_style { 550.0 } else { 495.0 },
    );
    let spacing = 30;
    let width = 1.0;
    let bottom_margin = 25;
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
            x1 = 400.0;
            black_layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
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
