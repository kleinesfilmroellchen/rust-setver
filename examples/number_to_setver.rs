use setver::SetVersion;
use std::env::args;
use std::rc::Rc;

fn main() {
	let mut args = args();
	args.next();

	if let Some(number_str) = args.next() {
		let number = number_str.trim().parse::<usize>().expect("version is not an integer");

		// This is exactly how natural numbers (including zero) work in set theory.
		// Some dynamic programming makes this ~ 1/2 O(n^2).
		let mut previous_number_setvers = Vec::with_capacity(number);
		previous_number_setvers.push(Rc::new(SetVersion::default()));

		for n in 1..=number {
			let mut n_version = SetVersion::default();
			for i in 0..n {
				n_version.add_child_version(previous_number_setvers[i].clone());
			}
			previous_number_setvers.push(Rc::new(n_version));
		}

		let number_version = previous_number_setvers.last().unwrap();
		println!("{}", number_version);
	} else {
		eprintln!("usage: number_to_setver NUMBER");
	}
}
