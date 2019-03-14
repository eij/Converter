extern crate walkdir;
extern crate threadpool;
extern crate gtk;

use std::{env, path::PathBuf};

mod structs;
mod globals;
mod utils;
mod drivers;
mod convert;
mod ui;

use crate::utils::threads;

fn main() {
	let path = PathBuf::from(env::args().last().unwrap());

	let n_threads = threads::guess_threads();

	if path.is_dir() {
		let mut p = convert::Process::new(n_threads);

		p.without_ui(&path);
	}
	else {
		ui::open(n_threads);
	}
}

