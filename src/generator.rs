use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn process(&self) {

	}

	pub fn save(&self) {
		// TODO: receive filename, duh
		self.current.save("output.jpg").unwrap();
	}
}

pub fn create(target_image: DynamicImage) -> Generator {
	let target = target_image.to_rgb();
	let current = RgbImage::new(target_image.dimensions().0, target_image.dimensions().1);
	let gen = Generator {
		target: target,
		current: current,
	};
	return gen;
}
