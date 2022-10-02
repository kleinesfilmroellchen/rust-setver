use setver::SetVersion;
use std::env::args;

fn main() {
	let mut args = args();
	args.next();

	if let Some(number_str) = args.next() {
		let number = number_str.trim().parse::<usize>().expect("version is not an integer");

		// This is exactly how natural numbers (including zero) work in set theory.
		// Some dynamic programming makes this ~ 1/2 O(n^2).
		let mut previous_number_setvers = Vec::with_capacity(number);
		previous_number_setvers.push(SetVersion::default());

		for n in 1..=number {
			let mut n_version = SetVersion::default();
			for child in previous_number_setvers.iter().take(n) {
				n_version.add_child_version(child.clone());
			}
			previous_number_setvers.push(n_version);
		}

		let number_version = previous_number_setvers.last().unwrap();
		println!("{}", number_version);
	} else {
		eprintln!("usage: number_to_setver NUMBER");
	}
}
