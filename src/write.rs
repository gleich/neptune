use std::fs::File;

use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use ordinal::Ordinal;
use printpdf::*;

use crate::document::Document;

pub fn rectangle(layer: &PdfLayerReference, x: Mm, y: Mm, width: Mm, height: Mm, solid: bool) {
    let points = vec![
        (Point::new(x, y), false),
        (Point::new(x + width, y), false),
        (Point::new(x + width, y + height), false),
        (Point::new(x, y + height), false),
    ];
    let line = Line {
        points,
        is_closed: true,
        has_fill: solid,
        has_stroke: true,
        is_clipping_path: false,
    };
    layer.add_shape(line);
}

pub fn days(document: &Document, x: Mm, y: Mm) {
    let weekday = Local::now().weekday().num_days_from_sunday() - 1;
    for (i, day) in vec!["M", "T", "W", "T", "F", "S", "S"].iter().enumerate() {
        let offset = Mm(30.0 * i as f64 + 1.0);
        let matches_weekday = weekday == i as u32;
        rectangle(
            if matches_weekday {
                &document.black_layer
            } else {
                &document.white_layer
            },
            x + offset,
            y,
            Mm(20.0),
            Mm(20.0),
            weekday == i as u32,
        );
        if matches_weekday {
            &document.white_layer
        } else {
            &document.black_layer
        }
        .use_text(
            day.to_string(),
            60.0,
            x + Mm(if *day == "T" {
                6.0
            } else if *day == "F" {
                5.5
            } else if *day == "S" {
                5.2
            } else {
                4.0
            }) + offset,
            y + Mm(4.0),
            &document.title_font,
        );
    }
}

pub fn name(document: &Document, x: Mm, y: Mm) {
    document.white_layer.use_text(
        "Matt Gleich",
        80.0,
        x + Mm(5.0),
        y + Mm(5.0),
        &document.title_font,
    );

    rectangle(&document.black_layer, x, y, Mm(130.0), Mm(26.0), true);
}

pub fn giant_date(document: &Document, x: Mm, y: Mm) -> f64 {
    let now = Local::now().day();
    let num_str = now.to_string();
    let mut ordinal_offset = if num_str.len() > 1 { 140.0 } else { 75.0 };
    if num_str.ends_with("7") {
        ordinal_offset += 7.0;
    }
    rectangle(
        &document.black_layer,
        x,
        y,
        Mm(30.0) + Mm(ordinal_offset),
        Mm(160.0),
        true,
    );
    document.white_layer.use_text(
        num_str,
        350.0,
        x + Mm(5.0),
        y + Mm(40.0),
        &document.giant_font,
    );
    document.white_layer.use_text(
        Ordinal(now).suffix(),
        60.0,
        x + Mm(ordinal_offset),
        y + Mm(100.0),
        &document.giant_font,
    );
    ordinal_offset
}

pub fn logo(document: &Document, x: Mm, y: Mm, scale: f64) -> Result<()> {
    let mut image = File::open("logo.jpg").context("Failed to read logo.png file")?;
    let image = Image::try_from(
        image_crate::codecs::jpeg::JpegDecoder::new(&mut image)
            .context("Failed to decode png logo")?,
    )
    .context("Failed to convert codecs to Image")?;
    image.add_to_layer(
        document.black_layer.to_owned(),
        ImageTransform {
            translate_x: Some(x),
            translate_y: Some(y),
            rotate: None,
            scale_x: Some(scale),
            scale_y: Some(scale),
            dpi: None,
        },
    );
    Ok(())
}
