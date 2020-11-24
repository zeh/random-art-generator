#[derive(Clone, Debug, PartialEq)]
pub enum FileFormat {
	PNG,
	JPEG,
}

impl FileFormat {
	pub fn from_filename(filename: &str) -> Result<FileFormat, &str> {
		if filename.ends_with(".png") {
			Ok(FileFormat::PNG)
		} else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
			Ok(FileFormat::JPEG)
		} else {
			Err("Invalid file format; only PNG and JPEG are accepted")
		}
	}
}
