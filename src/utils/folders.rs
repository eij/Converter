use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};

use crate::globals;

pub fn image_alloweds(entry: &DirEntry, formats: &Vec<String>) -> bool {
	if entry.file_type().is_file() {
		if formats.contains(&entry.path().extension().unwrap().to_str().unwrap().to_lowercase()) {
			(true)
		}
		else {
			#[cfg(feature = "verbose")]
			println!("   Skipping {}", entry.file_name().to_string_lossy());

			(false)
		}
	}
	else {
		(false)
	}
}

pub fn folder_disalloweds(entry: &DirEntry) -> bool {
	if entry.file_type().is_dir() {
		if globals::FOLDER_DISALLOWEDS.contains(&entry.file_name().to_str().unwrap()) {
			(false)
		}
		else {
			(true)
		}
	}
	else {
		(false)
	}
}

pub fn get_files_count(p: &PathBuf, formats: &Vec<String>) -> f64 {
	let mut c: f64 = 0.0;

	for _ in WalkDir::new(&p)
				.min_depth(2)
				.max_depth(2)
				.into_iter()
				.filter_map(|e| e.ok())
				.filter(|e| image_alloweds(e, &formats)) {
		c += 1.0;
	}

	(c)
}
