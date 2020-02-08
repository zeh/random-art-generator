use image::{RgbImage};

pub mod rect;
pub mod circle;

pub trait Painter {
	fn new() -> Self;
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}
