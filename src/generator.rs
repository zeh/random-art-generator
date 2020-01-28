use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage};
use rand::{random};

const LUMA_R: f64 = 0.2126;
const LUMA_G: f64 = 0.7152;
const LUMA_B: f64 = 0.0722;

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn from(target_image: DynamicImage) -> Generator {
		let target = target_image.to_rgb();
		let current = RgbImage::new(target_image.dimensions().0, target_image.dimensions().1);
		Generator {
			target: target,
			current: current,
		}
	}

	pub fn prepopulate(&mut self, current_image: DynamicImage) {
		self.current = current_image.to_rgb();
	}

	pub fn process(&mut self, candidates: u32, iterations: u32) {
		let painter = RectPainter {};

		let mut improved_iterations = 0;
		let mut discarded_iterations = 0;

		for i in 0..iterations {
			println!("Iteration {}", i + 1);

			let start_diff = Generator::diff(&self.current, &self.target);
			println!("  Starting diff is {}", start_diff);

			let mut best_candidate = painter.paint(&self.current);
			let mut best_diff = Generator::diff(&best_candidate, &self.target);
			println!("  Candidate {} diff is {}", 1, best_diff);
			println!("  Best new candidate!");

			for c in 1..candidates {
				let new_candidate = painter.paint(&self.current);
				let new_diff = Generator::diff(&new_candidate, &self.target);
				println!("  Candidate {} diff is {}", c + 1, new_diff);

				if new_diff < best_diff {
					println!("  Best new candidate!");
					best_candidate = new_candidate;
					best_diff = new_diff;
				}
			}

			if best_diff < start_diff {
				improved_iterations = improved_iterations + 1;
				println!("    Used new data.");
				self.current = best_candidate;
			} else {
				discarded_iterations = discarded_iterations + 1;
				println!("    Discarded new data; none of the candidates look good.");
			}
		}

		let final_diff = Generator::diff(&self.current, &self.target);
		println!("All iterations finished. Used {} iterations, and discarded {}. The final diff is {}.", improved_iterations, discarded_iterations, final_diff);
	}

	pub fn finalize(self) -> RgbImage {
		return self.current;
	}

	pub fn diff(a: &RgbImage, b: &RgbImage) -> f64 {
		let mut diff_sum: f64 = 0.0;
		let num_pixels: f64 = (a.dimensions().0 * a.dimensions().1) as f64;

		let mut diff_r: f64;
		let mut diff_g: f64;
		let mut diff_b: f64;

		for (x, y, pixel) in a.enumerate_pixels() {
			let p1 = pixel.channels4();
			let p2 = b[(x, y)].channels4();
			diff_r = (p1.0 as i16 - p2.0 as i16).abs() as f64 / 255.0;
			diff_g = (p1.1 as i16 - p2.1 as i16).abs() as f64 / 255.0;
			diff_b = (p1.2 as i16 - p2.2 as i16).abs() as f64 / 255.0;
			diff_sum += diff_r * LUMA_R + diff_g * LUMA_G + diff_b * LUMA_B;
		}

 		diff_sum / num_pixels
	}
}

trait Painter {
	fn new() -> Self;
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}

struct RectPainter {
}

impl Painter for RectPainter {
	fn new() -> RectPainter {
        RectPainter { }
	}

	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		//let rng = rand::thread_rng().gen_range(1, 101);
		let w = canvas.dimensions().0 as f32;
		let h = canvas.dimensions().1 as f32;
		let rect_w = (random::<f32>() * w).round() as u32;
		let rect_h = (random::<f32>() * h).round() as u32;
		let rect_x = (random::<f32>() * (w - rect_w as f32)).round() as u32;
		let rect_y = (random::<f32>() * (h - rect_h as f32)).round() as u32;
		// let pixel = image::RGB(255, 255, 255);
		// canvas.put_pixel(rect_x, rect_y, p);
		// return canvas;

		let r = random::<u8>();
		let g = random::<u8>();
		let b = random::<u8>();

		let painted_canvas = RgbImage::from_fn(w.round() as u32, h.round() as u32, |x, y| {
			if x >= rect_x && x <= rect_x + rect_w && y >= rect_y && y <= rect_y + rect_h {
				image::Rgb([r, g, b])
			} else {
				canvas[(x, y)]
			}
		});

		return painted_canvas
	}
}
