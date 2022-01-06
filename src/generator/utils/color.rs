use num_enum::IntoPrimitive;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, Hash, IntoPrimitive, PartialEq)]
#[repr(u8)]
pub enum BlendingMode {
	#[strum(serialize = "normal")]
	Normal = 0,
	#[strum(serialize = "multiply")]
	Multiply = 1,
	#[strum(serialize = "screen")]
	Screen = 2,
	#[strum(serialize = "overlay")]
	Overlay = 3,
	#[strum(serialize = "darken")]
	Darken = 4,
	#[strum(serialize = "lighten")]
	Lighten = 5,
	#[strum(serialize = "color-dodge")]
	ColorDodge = 6,
	#[strum(serialize = "color-burn")]
	ColorBurn = 7,
	#[strum(serialize = "hard-light")]
	HardLight = 8,
	#[strum(serialize = "soft-light")]
	SoftLight = 9,
	#[strum(serialize = "difference")]
	Difference = 10,
	#[strum(serialize = "exclusion")]
	Exclusion = 11,
}

impl Default for BlendingMode {
	fn default() -> Self {
		BlendingMode::Normal
	}
}

// Constants for pixel luminosity per channel
pub const LUMA_R: f64 = 0.2126;
pub const LUMA_G: f64 = 0.7152;
pub const LUMA_B: f64 = 0.0722;
