use std::collections::HashMap;

use image::RgbImage;

pub mod circle;
pub mod rect;
pub mod stroke;

pub trait Painter {
	fn paint(&self, canvas: &RgbImage, iteration: u64, seed_map: &RgbImage) -> Result<RgbImage, &str>;
	fn get_metadata(&self) -> HashMap<String, String>;
}
