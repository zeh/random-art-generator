use std::io::prelude::*;
use std::{fs::File, path::Path};

use bytes;
use bytes::{BufMut, BytesMut};
use image::{DynamicImage, ImageFormat, RgbImage};
use img_parts::{
	jpeg::{markers, Jpeg, JpegSegment},
	png::{Png, PngChunk},
	Bytes,
};
use structopt::clap::crate_version;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFileFormat {
	PNG,
	JPEG,
}

impl ImageFileFormat {
	pub fn from_path(path: &Path) -> Result<ImageFileFormat, &str> {
		let format = ImageFormat::from_path(&path).expect("reading image format from path");
		match format {
			ImageFormat::Png => Ok(ImageFileFormat::PNG),
			ImageFormat::Jpeg => Ok(ImageFileFormat::JPEG),
			_ => Err("Invalid file format; only PNG and JPEG are accepted"),
		}
	}

	pub fn get_native_format(&self) -> ImageFormat {
		match &self {
			ImageFileFormat::PNG => ImageFormat::Png,
			ImageFileFormat::JPEG => ImageFormat::Jpeg,
		}
	}
}

pub fn write_image(image_buffer: RgbImage, path: &Path) {
	let image = DynamicImage::ImageRgb8(image_buffer);
	let image_format = ImageFileFormat::from_path(path).expect("parsing image format");
	let mut output_file = File::create(path).expect("creating output file");
	// We could also have used "image.save(output_path)"
	image.write_to(&mut output_file, image_format.get_native_format()).unwrap();
}

pub fn generate_image(image_buffer: RgbImage, image_format: ImageFileFormat) -> Bytes {
	let image = DynamicImage::ImageRgb8(image_buffer);

	// Encode the image first
	let mut image_writer = BytesMut::new().writer();
	image.write_to(&mut image_writer, image_format.get_native_format()).expect("writing image to Bytes");
	Bytes::from(image_writer.into_inner().freeze().to_vec())
}

pub fn generate_image_with_metadata(
	image_buffer: RgbImage,
	image_format: ImageFileFormat,
	comments: Vec<String>,
) -> bytes::Bytes {
	let image_bytes = generate_image(image_buffer, image_format);
	let mut image_and_meta_writer = BytesMut::new().writer();

	// Additional metadata
	let meta_software = format!("Random Art Generator v{}", crate_version!());

	// Save differently based on file format
	match image_format {
		ImageFileFormat::PNG => {
			// Is PNG, add chunks
			let mut png = Png::from_bytes(image_bytes).expect("reading encoded PNG image");

			let comments_chunk =
				PngChunk::new(*b"tEXt", Bytes::from(format!("Comment\u{0}{}", comments.join(" \r\n"))));
			let software_chunk =
				PngChunk::new(*b"tEXt", Bytes::from(format!("Software\u{0}{}", meta_software)));

			let chunks = png.chunks_mut().len();
			png.chunks_mut().insert(chunks - 1, comments_chunk);
			png.chunks_mut().insert(chunks - 1, software_chunk);

			png.encoder().write_to(&mut image_and_meta_writer).expect("writing encoded PNG file");
		}
		ImageFileFormat::JPEG => {
			// Is JPEG, add segments
			let mut jpeg = Jpeg::from_bytes(image_bytes).expect("reading encoded JPEG image");

			let mut new_comments = comments.clone();
			new_comments.insert(0, meta_software);
			let comments_segment =
				JpegSegment::new_with_contents(markers::COM, Bytes::from(new_comments.join(" \r\n")));

			let segments = jpeg.segments_mut().len();
			jpeg.segments_mut().insert(segments - 1, comments_segment);

			jpeg.encoder().write_to(&mut image_and_meta_writer).expect("writing encoded JPEG file");
		}
	}

	bytes::Bytes::from(image_and_meta_writer.into_inner().freeze().to_vec())
}

pub fn write_image_with_metadata(image_buffer: RgbImage, path: &Path, comments: Vec<String>) {
	let image_format = ImageFileFormat::from_path(path).expect("parsing image format");
	let image_bytes = generate_image_with_metadata(image_buffer, image_format, comments);
	let mut output_file = File::create(path).expect("creating output file with metadata");
	output_file.write(&image_bytes[..]).expect("writing output file with metadata");
}
