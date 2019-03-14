extern crate num_cpus;

pub fn guess_threads() -> usize {
	let cpus = num_cpus::get();

	(cpus * 2)
}

