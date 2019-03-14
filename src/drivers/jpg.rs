extern crate image;

use std::fs::File;
use std::io::{self, Write, Seek, SeekFrom};
use std::path::PathBuf;

use self::image::{ImageOutputFormat, GenericImageView};

use crate::structs::Settings;

struct Dimension {
	w: u32,
	h: u32,
}

pub fn convert_to_jpg(entry: &PathBuf, destination: &PathBuf, settings: &Settings) -> Result<(), image::ImageError> {
	match image::open(entry) {
		Ok(image) => {
			let mut buffer = File::create(&destination).unwrap();

			let (w, h) = image.dimensions();

			let dimensions = get_dimensions_percentual(w, h, settings.resize_percentage);

			let _ = image
				.brighten(settings.adjustment_brightness)
				.resize(dimensions.w, dimensions.h, settings.resize_filter)
				.write_to(&mut buffer, ImageOutputFormat::JPEG(settings.image_quality));

			let _ = set_jfif_tags(&buffer, settings.adjustment_dpi);

			Ok(())
		},
		Err(e) => {
			Err(e)
		},
	}
}

fn get_dimensions_percentual(w: u32, h: u32, p: f32) -> Dimension {
	Dimension {
		w: ((p / 100.0) * w as f32) as u32,
		h: ((p / 100.0) * h as f32) as u32,
	}
}

fn set_jfif_tags(mut b: &File, d: u8) -> io::Result<()> {
	let bytes = [0xFF, 0xD8, 0xFF, 0xE0, 0x0, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x0, 0x1, 0x1, 0x1, 0x0, d, 0x0, d, 0x0, 0x0, 0xFF];

	b.seek(SeekFrom::Start(0))?;
	b.write_all(&bytes)?;

	Ok(())
}

