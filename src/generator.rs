use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub fn test() {
	let img = ImageBuffer::from_fn(512, 512, |x, _y| {
		if x % 2 == 0 {
			image::Luma([0u8])
		} else {
			image::Luma([255u8])
		}
	});

	img.save("output.png").unwrap();
}
