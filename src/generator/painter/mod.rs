use image::RgbImage;

pub mod circle;
pub mod rect;
pub mod stroke;

pub trait Painter {
	fn paint(&self, canvas: &RgbImage, seed_map: &RgbImage) -> RgbImage;
}
