use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum BlendingMode {
	#[strum(serialize = "normal")]
	Normal,
	#[strum(serialize = "multiply")]
	Multiply,
	#[strum(serialize = "screen")]
	Screen,
	#[strum(serialize = "overlay")]
	Overlay,
	#[strum(serialize = "darken")]
	Darken,
	#[strum(serialize = "lighten")]
	Lighten,
	#[strum(serialize = "color-dodge")]
	ColorDodge,
	#[strum(serialize = "color-burn")]
	ColorBurn,
	#[strum(serialize = "hard-light")]
	HardLight,
	#[strum(serialize = "soft-light")]
	SoftLight,
	#[strum(serialize = "difference")]
	Difference,
	#[strum(serialize = "exclusion")]
	Exclusion,
}

impl BlendingMode {
	#[inline(always)]
	pub fn blend(&self, bottom: f64, top: f64) -> f64 {
		match self {
			Self::Normal => top,
			Self::Multiply => bottom * top,
			Self::Screen => 1.0 - (1.0 - bottom) * (1.0 - top),
			Self::Overlay => {
				if bottom < 0.5 {
					2.0 * bottom * top
				} else {
					1.0 - 2.0 * (1.0 - bottom) * (1.0 - top)
				}
			}
			Self::Darken => bottom.min(top),
			Self::Lighten => bottom.max(top),
			Self::ColorDodge => {
				if bottom == 0.0 {
					0.0
				} else if top == 1.0 {
					1.0
				} else {
					(bottom / (1.0 - top)).min(1.0)
				}
			}
			Self::ColorBurn => {
				if bottom == 1.0 {
					1.0
				} else if top == 0.0 {
					0.0
				} else {
					1.0 - ((1.0 - bottom) / top).min(1.0)
				}
			}
			Self::HardLight => {
				if top <= 0.5 {
					2.0 * bottom * top
				} else {
					1.0 - (1.0 - bottom) * (1.0 - (2.0 * top - 1.0))
				}
			}
			Self::SoftLight => {
				if top <= 0.5 {
					bottom - (1.0 - 2.0 * top) * bottom * (1.0 - bottom)
				} else {
					let d = if bottom <= 0.25 {
						((16.0 * bottom - 12.0) * bottom + 4.0) * bottom
					} else {
						bottom.sqrt()
					};
					bottom + (2.0 * top - 1.0) * (d - bottom)
				}
			}
			Self::Difference => (bottom - top).abs().max(0.0).min(1.0),
			Self::Exclusion => bottom + top - 2.0 * bottom * top,
		}
	}

	/// Interpolates between the bottom color, and the resulting
	/// color if the top color was applied with this blend mode
	#[inline(always)]
	pub fn blend_with_opacity(&self, bottom: f64, top: f64, opacity: f64) -> f64 {
		return if opacity == 0.0 {
			bottom
		} else {
			let opaque_result = &self.blend(bottom, top);
			opaque_result * opacity + bottom * (1.0 - opacity)
		};
	}
}

impl Default for BlendingMode {
	fn default() -> Self {
		BlendingMode::Normal
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blend_normal() {
		// Opaque
		assert_eq!(BlendingMode::Normal.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend(0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend(0.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Normal.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend(0.5, 1.0), 1.0);

		assert_eq!(BlendingMode::Normal.blend(1.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend(1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend(1.0, 1.0), 1.0);

		// With transparency
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.0, 0.25), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.0, 0.75), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.0, 1.0), 0.0);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.5, 0.25), 0.125);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.5, 0.5), 0.25);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.5, 0.75), 0.375);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 0.5, 1.0), 0.5);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 1.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 1.0, 0.25), 0.25);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 1.0, 0.75), 0.75);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.0, 1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.0, 0.0), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.0, 0.25), 0.375);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.0, 0.5), 0.25);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.0, 0.75), 0.125);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.0, 1.0), 0.0);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.5, 0.25), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.5, 0.75), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 0.5, 1.0), 0.5);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 1.0, 0.0), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 1.0, 0.25), 0.625);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 1.0, 0.5), 0.75);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 1.0, 0.75), 0.875);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(0.5, 1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.0, 0.25), 0.75);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.0, 0.75), 0.25);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.0, 1.0), 0.0);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.5, 0.0), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.5, 0.25), 0.875);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.5, 0.5), 0.75);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.5, 0.75), 0.625);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 0.5, 1.0), 0.5);

		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 1.0, 0.25), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 1.0, 0.75), 1.0);
		assert_eq!(BlendingMode::Normal.blend_with_opacity(1.0, 1.0, 1.0), 1.0);
	}

	#[test]
	fn test_blend_multiply() {
		assert_eq!(BlendingMode::Multiply.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Multiply.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::Multiply.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::Multiply.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::Multiply.blend(0.5, 0.5), 0.25);
		assert_eq!(BlendingMode::Multiply.blend(0.5, 1.0), 0.5);
		assert_eq!(BlendingMode::Multiply.blend(1.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Multiply.blend(1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Multiply.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.25, 0.25, 0.25), 0.203125);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.25, 0.25, 0.75), 0.109375);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.25, 0.75, 0.25), 0.234375);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.25, 0.75, 0.75), 0.203125);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.75, 0.25, 0.25), 0.609375);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.75, 0.25, 0.75), 0.328125);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.75, 0.75, 0.25), 0.703125);
		assert_eq!(BlendingMode::Multiply.blend_with_opacity(0.75, 0.75, 0.75), 0.609375);
	}

	#[test]
	fn test_blend_screen() {
		assert_eq!(BlendingMode::Screen.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Screen.blend(0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Screen.blend(0.0, 1.0), 1.0);
		assert_eq!(BlendingMode::Screen.blend(0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::Screen.blend(0.5, 0.5), 0.75);
		assert_eq!(BlendingMode::Screen.blend(0.5, 1.0), 1.0);
		assert_eq!(BlendingMode::Screen.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Screen.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::Screen.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.25, 0.25, 0.25), 0.296875);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.25, 0.25, 0.75), 0.390625);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.25, 0.75, 0.25), 0.390625);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.25, 0.75, 0.75), 0.671875);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.75, 0.25, 0.25), 0.765625);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.75, 0.25, 0.75), 0.796875);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.75, 0.75, 0.25), 0.796875);
		assert_eq!(BlendingMode::Screen.blend_with_opacity(0.75, 0.75, 0.75), 0.890625);
	}

	#[test]
	fn test_blend_overlay() {
		assert_eq!(BlendingMode::Overlay.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Overlay.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::Overlay.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::Overlay.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::Overlay.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Overlay.blend(0.5, 1.0), 1.0);
		assert_eq!(BlendingMode::Overlay.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Overlay.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::Overlay.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.25, 0.25, 0.25), 0.21875);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.25, 0.25, 0.75), 0.15625);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.25, 0.75, 0.25), 0.28125);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.25, 0.75, 0.75), 0.34375);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.75, 0.25, 0.25), 0.71875);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.75, 0.25, 0.75), 0.65625);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.75, 0.75, 0.25), 0.78125);
		assert_eq!(BlendingMode::Overlay.blend_with_opacity(0.75, 0.75, 0.75), 0.84375);
	}

	#[test]
	fn test_blend_darken() {
		assert_eq!(BlendingMode::Darken.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Darken.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::Darken.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::Darken.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::Darken.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Darken.blend(0.5, 1.0), 0.5);
		assert_eq!(BlendingMode::Darken.blend(1.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Darken.blend(1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Darken.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.25, 0.25, 0.25), 0.25);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.25, 0.25, 0.75), 0.25);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.25, 0.75, 0.25), 0.25);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.25, 0.75, 0.75), 0.25);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.75, 0.25, 0.25), 0.625);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.75, 0.25, 0.75), 0.375);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.75, 0.75, 0.25), 0.75);
		assert_eq!(BlendingMode::Darken.blend_with_opacity(0.75, 0.75, 0.75), 0.75);
	}

	#[test]
	fn test_blend_lighten() {
		assert_eq!(BlendingMode::Lighten.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Lighten.blend(0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Lighten.blend(0.0, 1.0), 1.0);
		assert_eq!(BlendingMode::Lighten.blend(0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::Lighten.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Lighten.blend(0.5, 1.0), 1.0);
		assert_eq!(BlendingMode::Lighten.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Lighten.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::Lighten.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.25, 0.25, 0.25), 0.25);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.25, 0.25, 0.75), 0.25);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.25, 0.75, 0.25), 0.375);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.25, 0.75, 0.75), 0.625);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.75, 0.25, 0.25), 0.75);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.75, 0.25, 0.75), 0.75);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.75, 0.75, 0.25), 0.75);
		assert_eq!(BlendingMode::Lighten.blend_with_opacity(0.75, 0.75, 0.75), 0.75);
	}

	#[test]
	fn test_blend_color_dodge() {
		assert_eq!(BlendingMode::ColorDodge.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::ColorDodge.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::ColorDodge.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::ColorDodge.blend(0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::ColorDodge.blend(0.5, 0.5), 1.0);
		assert_eq!(BlendingMode::ColorDodge.blend(0.5, 1.0), 1.0);
		assert_eq!(BlendingMode::ColorDodge.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::ColorDodge.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::ColorDodge.blend(1.0, 1.0), 1.0);

		// These are a little bit different by +- 2 (on a 0-255 range)
		// when compared to Photopea results
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.25, 0.25, 0.25), 0.2708333333333333);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.25, 0.25, 0.75), 0.3125);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.25, 0.75, 0.25), 0.4375);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.25, 0.75, 0.75), 0.8125);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.75, 0.25, 0.25), 0.8125);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.75, 0.25, 0.75), 0.9375);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.75, 0.75, 0.25), 0.8125);
		assert_eq!(BlendingMode::ColorDodge.blend_with_opacity(0.75, 0.75, 0.75), 0.9375);
	}

	#[test]
	fn test_blend_color_burn() {
		assert_eq!(BlendingMode::ColorBurn.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::ColorBurn.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::ColorBurn.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::ColorBurn.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::ColorBurn.blend(0.5, 0.5), 0.0);
		assert_eq!(BlendingMode::ColorBurn.blend(0.5, 1.0), 0.5);
		assert_eq!(BlendingMode::ColorBurn.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::ColorBurn.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::ColorBurn.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.25, 0.25, 0.25), 0.1875);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.25, 0.25, 0.75), 0.0625);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.25, 0.75, 0.25), 0.1875);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.25, 0.75, 0.75), 0.0625);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.75, 0.25, 0.25), 0.5625);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.75, 0.25, 0.75), 0.1875);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.75, 0.75, 0.25), 0.7291666666666667);
		assert_eq!(BlendingMode::ColorBurn.blend_with_opacity(0.75, 0.75, 0.75), 0.6875);
	}

	#[test]
	fn test_blend_hard_light() {
		assert_eq!(BlendingMode::HardLight.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::HardLight.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::HardLight.blend(0.0, 1.0), 1.0);
		assert_eq!(BlendingMode::HardLight.blend(0.5, 0.0), 0.0);
		assert_eq!(BlendingMode::HardLight.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::HardLight.blend(0.5, 1.0), 1.0);
		assert_eq!(BlendingMode::HardLight.blend(1.0, 0.0), 0.0);
		assert_eq!(BlendingMode::HardLight.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::HardLight.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.25, 0.25, 0.25), 0.21875);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.25, 0.25, 0.75), 0.15625);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.25, 0.75, 0.25), 0.34375);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.25, 0.75, 0.75), 0.53125);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.75, 0.25, 0.25), 0.65625);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.75, 0.25, 0.75), 0.46875);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.75, 0.75, 0.25), 0.78125);
		assert_eq!(BlendingMode::HardLight.blend_with_opacity(0.75, 0.75, 0.75), 0.84375);
	}

	#[test]
	fn test_blend_soft_light() {
		assert_eq!(BlendingMode::SoftLight.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::SoftLight.blend(0.0, 0.5), 0.0);
		assert_eq!(BlendingMode::SoftLight.blend(0.0, 1.0), 0.0);
		assert_eq!(BlendingMode::SoftLight.blend(0.5, 0.0), 0.25);
		assert_eq!(BlendingMode::SoftLight.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::SoftLight.blend(0.5, 1.0), 0.7071067811865476);
		assert_eq!(BlendingMode::SoftLight.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::SoftLight.blend(1.0, 0.5), 1.0);
		assert_eq!(BlendingMode::SoftLight.blend(1.0, 1.0), 1.0);

		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.25, 0.25, 0.25), 0.2265625);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.25, 0.25, 0.75), 0.1796875);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.25, 0.75, 0.25), 0.28125);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.25, 0.75, 0.75), 0.34375);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.75, 0.25, 0.25), 0.7265625);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.75, 0.25, 0.75), 0.6796875);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.75, 0.75, 0.25), 0.7645031754730548);
		assert_eq!(BlendingMode::SoftLight.blend_with_opacity(0.75, 0.75, 0.75), 0.7935095264191645);
	}

	#[test]
	fn test_blend_difference() {
		assert_eq!(BlendingMode::Difference.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Difference.blend(0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Difference.blend(0.0, 1.0), 1.0);
		assert_eq!(BlendingMode::Difference.blend(0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::Difference.blend(0.5, 0.5), 0.0);
		assert_eq!(BlendingMode::Difference.blend(0.5, 1.0), 0.5);
		assert_eq!(BlendingMode::Difference.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Difference.blend(1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Difference.blend(1.0, 1.0), 0.0);

		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.25, 0.25, 0.25), 0.1875);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.25, 0.25, 0.75), 0.0625);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.25, 0.75, 0.25), 0.3125);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.25, 0.75, 0.75), 0.4375);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.75, 0.25, 0.25), 0.6875);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.75, 0.25, 0.75), 0.5625);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.75, 0.75, 0.25), 0.5625);
		assert_eq!(BlendingMode::Difference.blend_with_opacity(0.75, 0.75, 0.75), 0.1875);
	}

	#[test]
	fn test_blend_exclusion() {
		assert_eq!(BlendingMode::Exclusion.blend(0.0, 0.0), 0.0);
		assert_eq!(BlendingMode::Exclusion.blend(0.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Exclusion.blend(0.0, 1.0), 1.0);
		assert_eq!(BlendingMode::Exclusion.blend(0.5, 0.0), 0.5);
		assert_eq!(BlendingMode::Exclusion.blend(0.5, 0.5), 0.5);
		assert_eq!(BlendingMode::Exclusion.blend(0.5, 1.0), 0.5);
		assert_eq!(BlendingMode::Exclusion.blend(1.0, 0.0), 1.0);
		assert_eq!(BlendingMode::Exclusion.blend(1.0, 0.5), 0.5);
		assert_eq!(BlendingMode::Exclusion.blend(1.0, 1.0), 0.0);

		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.25, 0.25, 0.25), 0.28125);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.25, 0.25, 0.75), 0.34375);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.25, 0.75, 0.25), 0.34375);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.25, 0.75, 0.75), 0.53125);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.75, 0.25, 0.25), 0.71875);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.75, 0.25, 0.75), 0.65625);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.75, 0.75, 0.25), 0.65625);
		assert_eq!(BlendingMode::Exclusion.blend_with_opacity(0.75, 0.75, 0.75), 0.46875);
	}
}
