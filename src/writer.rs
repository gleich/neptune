use anyhow::Result;
use num::{NumCast, ToPrimitive};
use printpdf::{ImageTransform, Line, Mm, Point};

use crate::document::Document;
use crate::resources;

impl Document {
	pub fn draw_rectangle<X: ToPrimitive, Y: ToPrimitive, W: ToPrimitive, H: ToPrimitive>(
		&self,
		x: X,
		y: Y,
		width: W,
		height: H,
		solid: bool,
		black: bool,
	) {
		let (x, y, width, height) = (
			Mm(NumCast::from(x).unwrap()),
			Mm(NumCast::from(y).unwrap()),
			Mm(NumCast::from(width).unwrap()),
			Mm(NumCast::from(height).unwrap()),
		);
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
		if black {
			self.properties.black_layer.add_shape(line);
		} else {
			self.properties.white_layer.add_shape(line);
		}
	}

	pub fn draw_logo<X: ToPrimitive, Y: ToPrimitive, S: ToPrimitive>(
		&self,
		x: X,
		y: Y,
		scale: S,
	) -> Result<()> {
		let (x, y, scale) = (
			Mm(NumCast::from(x).unwrap()),
			Mm(NumCast::from(y).unwrap()),
			NumCast::from(scale).unwrap(),
		);
		resources::Images::load_logo()?.add_to_layer(
			self.properties.black_layer.to_owned(),
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
}
