use image::{RgbImage};

pub mod rect;
pub mod circle;

pub enum PainterType {
	Circle,
	Rect,
}

pub trait Painter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}

pub fn create_painter(painter_type: PainterType) -> Box<dyn Painter> {
	// let painter = rect::RectPainter::new();
	// let p = Box::Painter::new(painter);
	// return p;
	let painter: Box<dyn Painter> = match painter_type {
		PainterType::Circle => Box::new(circle::CirclePainter::new()),
		PainterType::Rect => Box::new(rect::RectPainter::new()),
		_ => Box::new(rect::RectPainter::new()),
	};
	painter
}
