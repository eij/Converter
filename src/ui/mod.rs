use std::path::PathBuf;
use std::sync::atomic::Ordering;

use gtk::prelude::*;
use gtk::{
	self,
	Builder,
	Button,
	CellRendererText,
	ComboBox,
	CheckButton,
	Entry,
	FileChooserDialog,
	ListBox,
	ProgressBar,
	TreeView,
	TreeStore,
	TreeViewColumn,
	Scale,
	SpinButton,
	Switch,
	Window,
	WindowType
};

use crate::convert;
use crate::structs::{Settings, UIComponents};

fn show_select_dialog(directory_entry: &Entry) {
	let dialog = FileChooserDialog::new (
		Some("Choose Directory"),
		Some(&Window::new(WindowType::Popup)),
		gtk::FileChooserAction::SelectFolder,
	);

	dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
	dialog.add_button("Select", gtk::ResponseType::Ok.into());

	if dialog.run() == gtk::ResponseType::Ok.into() {
		dialog.get_filename().map(|path| path.to_str().map(|text| directory_entry.set_text(text)));

		dialog.destroy();
	}
}

pub fn open(n_threads: usize) {
	gtk::init().unwrap_or_else(|_| panic!("panic!"));

	let builder = Builder::new_from_string(include_str!("window.ui"));

	//	Main

	let btn_convert: Button						= builder.get_object("btn_convert").unwrap();
	let btn_stop: Button						= builder.get_object("btn_stop").unwrap();
	let btn_close: Button						= builder.get_object("btn_close").unwrap();
	let directory_entry: Entry					= builder.get_object("directory_entry").unwrap();
	let btn_directory: Button					= builder.get_object("btn_directory").unwrap();
	let progressbar: ProgressBar 					= builder.get_object("convert_progressbar").unwrap();

	//

	let listbox_folders: ListBox					= builder.get_object("listbox_folders").unwrap();

	//	Preferences

	let btn_preferences: Button					= builder.get_object("btn_preferences").unwrap();
	let btn_preferences_close: Button				= builder.get_object("btn_preferences_close").unwrap();

	let switch_recompress: Switch					= builder.get_object("switch_recompress").unwrap();
	let sld_recompress_image_quality: Scale 			= builder.get_object("sld_recompress_image_quality").unwrap();
	let spin_recompress_limit: SpinButton				= builder.get_object("spin_recompress_limit").unwrap();

	let sld_threads: Scale 						= builder.get_object("sld_threads").unwrap();
	let sld_image_quality: Scale 					= builder.get_object("sld_image_quality").unwrap();
	let sld_resize_percentage: Scale 				= builder.get_object("sld_resize_percentage").unwrap();

	//	Preferences > General

	let switch_guess_threads: Switch				= builder.get_object("switch_guess_threads").unwrap();
	let switch_folder_ordered: Switch				= builder.get_object("switch_folder_ordered").unwrap();

	//	Preferences > Compression

	let switch_image_quality: Switch				= builder.get_object("switch_image_quality").unwrap();

	//	Preferences > Adjustment

	let sld_brightness: Scale 					= builder.get_object("sld_brightness").unwrap();
	let spin_adjustment_dpi: SpinButton				= builder.get_object("spin_adjustment_dpi").unwrap();

	let combo_resize_filter: ComboBox				= builder.get_object("combo_resize_filter").unwrap();

	//	Windows

	let window = create_main_window(&builder);
	let window_preferences = create_preferences_window(&builder);

	window.show_all();

	let window_preferences_clone = window_preferences.clone();

	combo_add_cell(&combo_resize_filter, 0);
	combo_add_cell(&combo_resize_filter, 1);

	combo_resize_filter.set_active(0);

	let directory_clone = directory_entry.clone();

	let store_errors = create_errors_store(&builder);

	let sld_threads_clone = sld_threads.clone();

	switch_guess_threads.connect_property_active_notify(move |_| {
		if sld_threads_clone.get_sensitive() {
			sld_threads_clone.set_sensitive(false);
		}
		else {
			sld_threads_clone.set_sensitive(true);
		}
	});

	btn_preferences.connect_clicked(move |_| {
		window_preferences.show();
	});

	btn_preferences_close.connect_clicked(move |_| {
		window_preferences_clone.hide();
	});

	btn_directory.connect_clicked(move |_| {
		show_select_dialog(&directory_entry);
	});

	btn_convert.connect_clicked(move |btn_convert| {
		if directory_clone.get_text() == Some("".to_string()) {
			return
		}

		btn_convert.set_sensitive(false);
		btn_preferences.set_sensitive(false);
		btn_directory.set_sensitive(false);
		directory_clone.set_sensitive(false);
		btn_stop.set_sensitive(true);

		let mut p = convert::Process::new(match switch_guess_threads.get_active() {
			true => n_threads,
			false => sld_threads.get_value() as usize,
		});

		p.with_ui(&directory_clone.clone().get_text().unwrap(), &get_formats(&builder),
			Settings {
				path: PathBuf::new(),
				destination: PathBuf::new(),
				image_quality: match switch_image_quality.get_active() {
						true => sld_image_quality.get_value() as u8,
						false => 100,
					},
				recompress: switch_recompress.get_state(),
				recompress_limit: spin_recompress_limit.get_value() as u64,
				recompress_image_quality: sld_recompress_image_quality.get_value() as u8,
				resize_percentage: sld_resize_percentage.get_value() as f32,
				resize_filter: match combo_resize_filter.get_active() {
						0 => image::CatmullRom,
						1 => image::Lanczos3,
						_ => image::CatmullRom,
				},
				adjustment_brightness: sld_brightness.get_value() as i32,
				adjustment_dpi: spin_adjustment_dpi.get_value() as u8,
				folder_ordered: switch_folder_ordered.get_state(),
			},
			UIComponents {
				folder: listbox_folders.clone(),
				errors: store_errors.clone(),
				bar: progressbar.clone(),
			}
		);

		btn_stop.connect_clicked(move |b| {
			b.set_sensitive(false);

			//	TODO: use p.stop() instead accessing the variable directly
			p.stop.store(true, Ordering::Relaxed);
		});
	});

	btn_close.connect_clicked(move |_| {
		window.destroy();
		gtk::main_quit();
	});

	gtk::main();
}

fn create_main_window(builder: &Builder) -> Window {
	let window: Window = builder.get_object("window").unwrap();

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	window
}

fn create_preferences_window(builder: &Builder) -> Window {
	let window: Window = builder.get_object("window_pref").unwrap();

	window.connect_delete_event(|w, _| {
		w.hide();
		Inhibit(true)
	});

	window
}

fn create_errors_store(builder: &Builder) -> TreeStore {
	let tree_errors: TreeView = builder.get_object("tree_errors").unwrap();
	let store_errors = TreeStore::new(&[String::static_type(), String::static_type()]);

	tree_errors.set_model(Some(&store_errors));
	tree_errors.set_headers_visible(true);

	create_column(&tree_errors, "Thread", 0);
	create_column(&tree_errors, "Info", 1);

	store_errors
}

fn get_formats(builder: &Builder) -> Vec<String> {
	let chk_jpg: CheckButton					= builder.get_object("chk_jpg").unwrap();
	let chk_jpeg: CheckButton					= builder.get_object("chk_jpeg").unwrap();
	let chk_png: CheckButton					= builder.get_object("chk_png").unwrap();
	let chk_gif: CheckButton					= builder.get_object("chk_gif").unwrap();
	let chk_tif: CheckButton					= builder.get_object("chk_tif").unwrap();
	let chk_tiff: CheckButton					= builder.get_object("chk_tiff").unwrap();

	let mut formats = Vec::new();

	if chk_jpg.get_active() {
		formats.push("jpg".to_string());
	}

	if chk_jpeg.get_active() {
		formats.push("jpeg".to_string());
	}

	if chk_png.get_active() {
		formats.push("png".to_string());
	}

	if chk_gif.get_active() {
		formats.push("gif".to_string());
	}

	if chk_tif.get_active() {
		formats.push("tif".to_string());
	}

	if chk_tiff.get_active() {
		formats.push("tiff".to_string());
	}

	formats
}

fn combo_add_cell(combo: &ComboBox, position: i32) {
	let cell = CellRendererText::new();

	combo.pack_start(&cell, true);
	combo.add_attribute(&cell, "text", position);
}

fn create_column(tree: &TreeView, title: &str, position: i32) {
	let col = TreeViewColumn::new();
	let cell = CellRendererText::new();

	col.pack_start(&cell, true);
	col.add_attribute(&cell, "text", position);
	col.set_title(title);
	col.set_resizable(true);

	tree.append_column(&col);
}

