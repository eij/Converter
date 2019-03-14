pub static FOLDER_DISALLOWEDS: &[&str; 2] = &[ "JPG", "." ];

pub static IMAGE_QUALITY: u8 = 82;

pub static IMAGE_RESIZE_PERCENTAGE: f32 = 50.0;
pub static IMAGE_RESIZE_FILTER: image::FilterType = image::CatmullRom;

pub static IMAGE_ADJUST_BRIGHTNESS: i32 = -2;

pub static RECOMPRESS: bool = true;
pub static RECOMPRESS_LIMIT: u64 = 1_000_000;
pub static RECOMPRESS_IMAGE_QUALITY: u8 = 75;

