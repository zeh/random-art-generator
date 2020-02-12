use image::{RgbImage};

pub mod rect;
pub mod circle;

pub trait Painter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}
