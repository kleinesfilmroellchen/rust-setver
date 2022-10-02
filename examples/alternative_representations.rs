
use setver::SetVersion;
use std::env::args;

fn main() {
	let mut args = args();
	args.next();
	if let Some(version) = args.next() {
		let canonicalized = version.parse::<SetVersion>().expect("invalid setver version");
		let canonicalized_str = canonicalized.to_string();
		let original_width = version.len().max("direct".len());
		let canonical_width = canonicalized_str.len().max("canonicalized".len());
		println!(
			"                    {:>original_width$} {:>canonical_width$}
set representation  {:>original_width$} {:>canonical_width$}
integralternative   {:>original_width$} {:>canonical_width$}",
			"direct",
			"canonicalized",
			version,
			canonicalized_str,
			SetVersion::string_to_integralternative(&version),
			canonicalized.to_integralternative(),
			original_width = original_width,
			canonical_width = canonical_width
		);
	} else {
		eprintln!("usage: alternative_representations SETVER_VERSION");
	}
}
