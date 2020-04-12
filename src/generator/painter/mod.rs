use image::RgbImage;

pub mod circle;
pub mod rect;

pub trait Painter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}
