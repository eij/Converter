use std::path::PathBuf;
use std::os::linux::fs::MetadataExt;
use std::fs::{self, File};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Instant;

use std::collections::HashMap;

use gtk::{ProgressBarExt, TreeStoreExtManual};

use walkdir::WalkDir;

use crate::structs::{GLOBAL, Settings, Content};

use crate::utils::folders;

use crate::drivers::jpg;

pub fn single(stop: Arc<AtomicBool>, formats: Vec<String>, mut settings: Settings, sender: Option<&Sender<Content>>) {
	let partial_folder_count: f64 = folders::get_files_count(&settings.path, &formats);
	let mut partial_file_count: f64 = 0.0;

	for e in WalkDir::new(&settings.path).into_iter().filter_map(|e| e.ok()).filter(|e| folders::image_alloweds(e, &formats)) {
		if stop.load(Ordering::Relaxed) == true {
			break;
		}

		partial_file_count += 1.0;

		let mut c = Content{
			thread: format!("{:?}", thread::current().id()),
			path: String::new(),
			entry: String::new(),
			time: String::new(),
			errors: String::new(),
			progress: HashMap::new(),
		};

		let duration = Instant::now();

		let p = e.path().to_path_buf();

		fs::create_dir_all(&settings.destination).expect("Can't create the directory");

		let file = settings.destination.join(p.file_stem().unwrap().to_str().unwrap().to_owned() + ".jpg");

		match jpg::convert_to_jpg(&p, &file, &settings) {
			Ok(_) => {
				if settings.recompress {
					let buffer = File::open(PathBuf::from(&file)).unwrap();

					if buffer.metadata().unwrap().st_size() >= settings.recompress_limit {
						settings.image_quality = settings.recompress_image_quality;

						let _ = jpg::convert_to_jpg(&p, &file, &settings);
					}
				}

				c.progress.insert(
					format!("{}", settings.path.file_name().unwrap().to_string_lossy()),
					partial_file_count / partial_folder_count,
				);

				c.path.push_str(&format!("{}", settings.path.file_name().unwrap().to_string_lossy()));
				c.entry.push_str(&format!("{}", e.file_name().to_string_lossy()));
			},
			Err(_) => {
				c.errors.push_str(&format!("Can't convert {}", p.to_string_lossy()));
			},
		}

		c.time.push_str(&format!("{:?}ms", duration.elapsed().subsec_millis()));

		if sender.is_some() {
			sender.unwrap().send(c).unwrap();

			glib::idle_add(update_ui);
		}
		else {
			if !c.entry.is_empty() {
				println!("{} => {}\t\t{} [{}]", c.thread, c.path, c.entry, c.time);
			}
		}
	}
}

fn update_ui() -> glib::Continue {
	GLOBAL.with(|global| {
		if let Some((ref c, ref s, ref rx)) = *global.borrow() {
			if let Ok(content) = rx.try_recv() {
/*
				if content.entry.len() > 0 {
					c.content.insert_with_values(None, None, &[0, 1, 2, 3], &[
						&content.thread,
						&content.path,
						&content.entry,
						&content.time
					]);

println!("--> {:?}", c.folder);

					//println!("--> {:?}", content.progress);
				}
*/

				if content.errors.len() > 0 {
					c.errors.insert_with_values(None, None, &[0, 1], &[&content.thread, &content.errors]);
				}

				c.bar.set_fraction(c.bar.get_fraction() + s);
			}
		}
	});

	glib::Continue(false)
}
