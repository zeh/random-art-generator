use image::{RgbImage};
use rand::{random};

pub trait Painter {
	fn new() -> Self;
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}

pub struct RectPainter {
}

impl Painter for RectPainter {
	fn new() -> RectPainter {
		RectPainter { }
	}

	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		//let rng = rand::thread_rng().gen_range(1, 101);
		let w = canvas.dimensions().0 as f32;
		let h = canvas.dimensions().1 as f32;
		let random_w = random::<f32>();
		let random_h = random::<f32>();
		let rect_w = random_w * random_w * random_w * w;
		let rect_h = random_h * random_h * random_h * h;
		let rect_x = random::<f32>() * (w - rect_w);
		let rect_y = random::<f32>() * (h - rect_h);

		let x1 = rect_x as u32;
		let x2 = (rect_x + rect_w) as u32;
		let y1 = rect_y as u32;
		let y2 = (rect_y + rect_h) as u32;

		let r = random::<u8>();
		let g = random::<u8>();
		let b = random::<u8>();

		let pixel = image::Rgb([r, g, b]);

		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				painted_canvas.put_pixel(x, y, pixel);
			}
		}

		painted_canvas
	}
}
