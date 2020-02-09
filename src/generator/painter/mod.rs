use image::{RgbImage};

pub mod rect;
pub mod circle;

pub enum PainterType {
	Circle,
	Rect,
}

pub trait Painter {
	fn new() -> Self;
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}

pub fn create_painter(painter_type: PainterType) -> impl Painter {
	// if painter_type == PainterType::Rect {
	// 	return rect::RectPainter::new();
	// } else {
	// 	return rect::RectPainter::new();
	// }
	let painter = match painter_type {
		PainterType::Circle => circle::CirclePainter::new(),
		PainterType::Rect => rect::RectPainter::new(),
		_ => rect::RectPainter::new(),
	};
	painter
	// p
}
