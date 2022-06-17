use std::fs::File;

use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate};
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

pub fn days(document: &Document, x: Mm, y: Mm, date: DateTime<Local>) {
    let weekday = date.weekday().num_days_from_sunday() - 1;
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

pub fn giant_date(document: &Document, x: Mm, y: Mm, date: DateTime<Local>) -> f64 {
    let now = date.day();
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

pub fn date_information(document: &Document, x: Mm, y: Mm, date: DateTime<Local>) {
    document.black_layer.use_text(
        date.format("%B").to_string(),
        125.0,
        x,
        y + Mm(100.0),
        &document.title_font,
    );
    document.black_layer.use_text(
        date.year().to_string(),
        125.0,
        x,
        y + Mm(70.0),
        &document.title_font,
    );
    rectangle(
        &document.black_layer,
        x,
        y + Mm(63.0),
        Mm(160.0),
        Mm(3.0),
        true,
    );
    let day_of_year = date.format("%j").to_string().parse::<i32>().unwrap();
    let week_of_year = date.format("%U").to_string().parse::<i32>().unwrap();
    let year = date.year();
    let month = date.month();
    document.black_layer.use_text(
        format!(
            "{} day of year [{}%]",
            Ordinal(day_of_year),
            ((day_of_year as f32 / 366.0) * 100.0).round()
        ),
        50.0,
        x,
        y + Mm(45.0),
        &document.title_font,
    );
    document.black_layer.use_text(
        format!(
            "{} day of month [{}%]",
            Ordinal(date.day()),
            (date.day() as f32
                / NaiveDate::from_ymd(
                    match month {
                        12 => year + 1,
                        _ => year,
                    },
                    match month {
                        12 => 1,
                        _ => month + 1,
                    },
                    1,
                )
                .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
                .num_days() as f32
                * 100.0)
                .round()
        ),
        50.0,
        x,
        y + Mm(30.0),
        &document.title_font,
    );
    document.black_layer.use_text(
        format!(
            "{} day of week [{}%]",
            Ordinal(date.weekday() as usize + 1),
            (((date.weekday() as usize) as f32 + 1.0) / 7.0 * 100.0).round()
        ),
        50.0,
        x,
        y + Mm(15.0),
        &document.title_font,
    );
    document.black_layer.use_text(
        format!(
            "{} week of year [{}%]",
            Ordinal(week_of_year),
            (week_of_year as f32 / 53.0 * 100.0).round()
        ),
        50.0,
        x,
        y,
        &document.title_font,
    );
}
