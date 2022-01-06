use super::units::{Margins, Rectangle, SizeUnit};

pub fn find_target_draw_rect(
	dimensions: (u32, u32),
	margins: &Margins<SizeUnit>,
) -> Result<Rectangle<i64>, &'static str> {
	let pixel_margins = margins.to_pixels(dimensions.0, dimensions.1);
	let mut rect = Rectangle::<i64> {
		x: 0,
		y: 0,
		width: dimensions.0 as i64,
		height: dimensions.1 as i64,
	};
	rect.apply_margins(pixel_margins);
	if rect.width <= 0 || rect.height <= 0 {
		Err("Cannot have a resulting rectangle of negative or empty area")
	} else {
		Ok(rect)
	}
}
