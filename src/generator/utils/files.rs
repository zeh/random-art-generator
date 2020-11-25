use image::ImageFormat;

#[derive(Clone, Debug, PartialEq)]
pub enum FileFormat {
	PNG,
	JPEG,
}

impl FileFormat {
	pub fn from_filename(filename: &str) -> Result<FileFormat, &str> {
		let format = ImageFormat::from_path(&filename).unwrap();
		match format {
			ImageFormat::Png => Ok(FileFormat::PNG),
			ImageFormat::Jpeg => Ok(FileFormat::JPEG),
			_ => Err("Invalid file format; only PNG and JPEG are accepted"),
		}
	}

	pub fn get_native_format(&self) -> ImageFormat {
		match &self {
			FileFormat::PNG => ImageFormat::Png,
			FileFormat::JPEG => ImageFormat::Jpeg,
		}
	}
}
