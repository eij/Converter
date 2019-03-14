use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use walkdir::WalkDir;
use threadpool::ThreadPool;

use gtk::{ContainerExt, BoxExt, LabelExt, WidgetExt};

use crate::structs::{GLOBAL, Settings, UIComponents};

use crate::globals;

use crate::utils::folders;

mod single;

pub struct Process {
	pub pool: ThreadPool,
	pub stop: Arc<AtomicBool>,
}

impl Process {

	pub fn new(n_threads: usize) -> Process {
		Process {
			pool: ThreadPool::new(n_threads),
			stop: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn without_ui(&mut self, path: &PathBuf) {
		for entry in WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().filter_entry(|e| folders::folder_disalloweds(e)).filter_map(|e| e.ok()) {
			let settings = Settings {
				path: entry.path().to_path_buf(),
				destination: path.join("JPG").join(entry.file_name()),
				image_quality: globals::IMAGE_QUALITY,
				recompress: globals::RECOMPRESS,
				recompress_limit: globals::RECOMPRESS_LIMIT,
				recompress_image_quality: globals::RECOMPRESS_IMAGE_QUALITY,
				resize_percentage: globals::IMAGE_RESIZE_PERCENTAGE,
				resize_filter: globals::IMAGE_RESIZE_FILTER,
				adjustment_brightness: globals::IMAGE_ADJUST_BRIGHTNESS,
				adjustment_dpi: 150,
				folder_ordered: false,
			};

			let stop = self.stop.clone();

			self.pool.execute(move || {
				single::single(stop, Vec::new(), settings, None);
			});
		}

		self.pool.join();
	}

	pub fn with_ui(&mut self, path_t: &String, formats: &Vec<String>, mut settings: Settings, components: UIComponents) {
		let path = PathBuf::from(&path_t);

		let step = 1.0 / folders::get_files_count(&path, &formats);

		let (tx, rx) = channel();

		GLOBAL.with(|global| {
			*global.borrow_mut() = Some((components, step, rx))
		});

		let mut iterator = WalkDir::new(&path).min_depth(1).max_depth(1);

		if settings.folder_ordered {
			iterator = iterator.sort_by(|a, b| a.file_name().cmp(b.file_name()));
		}

		for entry in iterator.into_iter().filter_entry(|e| folders::folder_disalloweds(e)).filter_map(|e| e.ok()) {
			settings.path = entry.path().to_path_buf();
			settings.destination = path.join("JPG").join(entry.file_name());

			GLOBAL.with(|global| {
				if let Some((ref c, ref _s, ref _rx)) = *global.borrow() {
					match entry.path().file_name() {
						Some(a) => {
							let b = gtk::ListBoxRow::new();
							let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
							b.add(&hbox);

							let lab = gtk::Label::new(None);
							lab.set_text(&a.to_string_lossy());
							hbox.pack_start(&lab, true, true, 0);

							hbox.pack_start(&gtk::ProgressBar::new(), true, true, 0);

							c.folder.add(&b);
							c.folder.show_all();
						},
						None => {},
					}
				}
			});

			let stop = self.stop.clone();

			let f = formats.clone();

			let s = settings.clone();

			let sender = tx.clone();

			self.pool.execute(move || {
				single::single(stop, f, s, Some(&sender));
			});
		}
	}

	pub fn stop(&mut self) {
		self.stop.store(false, Ordering::Relaxed);
	}

}

