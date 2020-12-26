use std::collections::HashMap;

use image::{GrayImage, RgbImage};

pub mod circle;
pub mod rect;
pub mod stroke;

pub trait Painter {
	fn paint(
		&self,
		canvas: &RgbImage,
		iteration: u32,
		seed_map: &RgbImage,
		focus_map: &GrayImage,
	) -> Result<RgbImage, &str>;
	fn get_metadata(&self) -> HashMap<String, String>;
}
