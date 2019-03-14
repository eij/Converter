#![allow(non_snake_case)]
use std::path::PathBuf;
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;

#[derive(Clone)]
pub struct Settings {
	pub path: PathBuf,
	pub destination: PathBuf,
	pub image_quality: u8,
	pub recompress: bool,
	pub recompress_limit: u64,
	pub recompress_image_quality: u8,
	pub resize_percentage: f32,
	pub resize_filter: image::FilterType,
	pub adjustment_brightness: i32,
	pub adjustment_dpi: u8,
	pub folder_ordered: bool,
}

#[derive(Debug)]
pub struct Content {
	pub thread: String,
	pub path: String,
	pub entry: String,
	pub time: String,
	pub errors: String,
	pub progress: HashMap<String, f64>,
}

pub struct UIComponents {
	pub folder: gtk::ListBox,
	pub errors: gtk::TreeStore,
	pub bar: gtk::ProgressBar,
}

thread_local!(
	pub static GLOBAL: RefCell<Option<(UIComponents, f64, Receiver<Content>)>> = RefCell::new(None)
);

