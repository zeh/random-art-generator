use image::{DynamicImage, GenericImageView, Pixel, RgbImage};
use rand::{random};
use std::time::{Instant};

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

	pub fn process(&mut self, iterations: u32) {
		let painter = RectPainter {};

		let mut improved_iterations = 0;
		let mut discarded_iterations = 0;

		let mut new_candidate;
		let mut new_diff;
		let mut curr_diff = Generator::diff(&self.current, &self.target);

		println!("Starting iterations with a diff of {:.2}%.", curr_diff * 100.0);

		let mut time_start;
		let mut time_elapsed;
		let mut time_elapsed_paint;
		let mut time_elapsed_total: u128 = 0;

		for i in 0..iterations {
			time_start = Instant::now();

			new_candidate = painter.paint(&self.current);
			time_elapsed_paint = time_start.elapsed().as_millis();

			new_diff = Generator::diff(&new_candidate, &self.target);

			print!("Iteration {}/{} : diff is {:>5.2}%;", i + 1, iterations, new_diff * 100.0);

			if new_diff < curr_diff {
				improved_iterations = improved_iterations + 1;
				print!(" used.");
				self.current = new_candidate;
				curr_diff = new_diff;
			} else {
				discarded_iterations = discarded_iterations + 1;
				print!(" discarded.");
			}

			time_elapsed = time_start.elapsed().as_millis();
			println!(" ({}ms paint, {}ms total)", time_elapsed_paint, time_elapsed);

			time_elapsed_total = time_elapsed_total + time_elapsed;
		}

		let final_diff = Generator::diff(&self.current, &self.target);
		println!("Finished in {}ms ({}ms avg per iteration).", time_elapsed_total, time_elapsed_total / iterations as u128);
		println!("Used {} iterations, and discarded {}.", improved_iterations, discarded_iterations);
		println!("The final diff from target is {:.2}%.", final_diff * 100.0);
	}

	pub fn finalize(self) -> RgbImage {
		return self.current;
	}

	pub fn diff(a: &RgbImage, b: &RgbImage) -> f64 {
		let w = a.dimensions().0;
		let h = a.dimensions().1;
		let num_pixels: f64 = (w * h) as f64;

		let mut diff_sum_r: i64 = 0;
		let mut diff_sum_g: i64 = 0;
		let mut diff_sum_b: i64 = 0;

		let mut p1;
		let mut p2;

		for (x, y, pixel) in a.enumerate_pixels() {
			p1 = pixel.channels();
			p2 = b[(x, y)].channels();
			diff_sum_r += (p1[0] as i64 - p2[0] as i64).abs();
			diff_sum_g += (p1[1] as i64 - p2[1] as i64).abs();
			diff_sum_b += (p1[2] as i64 - p2[2] as i64).abs();
		}

		let lr = LUMA_R / 255.0;
		let lg = LUMA_G / 255.0;
		let lb = LUMA_B / 255.0;
		let diff_sum = diff_sum_r as f64 * lr + diff_sum_g as f64 * lg + diff_sum_b as f64 * lb;

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
		let random_w = random::<f32>();
		let random_h = random::<f32>();
		let rect_w = random_w * random_w * random_w * w;
		let rect_h = random_h * random_h * random_h * h;
		let rect_x = random::<f32>() * (w - rect_w);
		let rect_y = random::<f32>() * (h - rect_h);

		let x1 = rect_x as u32;
		let x2 = (rect_x + rect_w) as u32 + 1;
		let y1 = rect_y as u32;
		let y2 = (rect_y + rect_h) as u32 + 1;

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

		return painted_canvas
	}
}
