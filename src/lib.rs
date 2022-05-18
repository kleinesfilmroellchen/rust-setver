//! SetVer comprehension for Rust.

#![deny(missing_docs, clippy::all)]

use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

/// A SetVer version specification.
/// # Implementation details
/// This struct is implemented using HashSet from the standard library.
/// Therefore, it is not usable in no-std environments right now.
#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd, Default)]
pub struct SetVersion {
	/// Making this an ordered set guarantees that all iterations are performed in order, which gives some nice guarantees for faster implementations.
	versions: BTreeSet<SetVersion>,
}

impl SetVersion {
	/// Implements the SetVer comparison function, as per [the specification](https://github.com/RocketRace/setver#version-comparison-comparison).
	///
	/// The only returned values are 0, 1, Infinity and NaN.
	pub fn setver_compare(&self, other: &SetVersion) -> f32 {
		if self.is_subset(other) {
			0.0
		} else if self == other {
			1.0
		} else if other.is_subset(self) {
			f32::INFINITY
		} else {
			f32::NAN
		}
	}

	/// Returns whether this SetVer version is a subset of the other version, according to standard set laws.
	pub fn is_subset(&self, other: &SetVersion) -> bool {
		self.versions.is_subset(&other.versions)
	}

	/// Returns whether this SetVer version is a strict subset of the other version, according to standard set laws.
	pub fn is_strict_subset(&self, other: &SetVersion) -> bool {
		!other.is_superset(self)
	}
	/// Returns whether this SetVer version is a superset of the other version, according to standard set laws.
	pub fn is_superset(&self, other: &SetVersion) -> bool {
		self.versions.is_superset(&other.versions)
	}

	/// Returns whether this SetVer version is a strict superset of the other version, according to standard set laws.
	pub fn is_strict_superset(&self, other: &SetVersion) -> bool {
		!other.is_subset(self)
	}
	/// Implements the [Integralternative](https://github.com/RocketRace/setver#the-integralternative). This is the same as converting a `SetVersion` into an `u128`.
	/// # Panics
	/// For obvious reasons (if you know how the integralternative works), SetVersions with more than 128 braces in their text representation cannot be stored in a u128.
	/// In this case, this function will panic. Use `to_integralternative_bytes` instead.
	pub fn to_integralternative(&self) -> u128 {
		self.into()
	}

	/// Implements the [Integralternative](https://github.com/RocketRace/setver#the-integralternative).
	/// The return value is not as practical as the one of `to_integralternative`, but it can always represent the integralternative and will never panic.
	///
	/// The returned bytes are in LSB-first order (little-endian).
	pub fn to_integralternative_bytes(&self) -> Vec<u8> {
		// Could be done more efficiently, but this saves us a bunch of code.
		let stringified = String::from(self);
		let mut current_byte = 0;
		let mut bytes = Vec::new();
		let mut bit_count = 0;
		for c in stringified.chars().rev() {
			current_byte = (current_byte >> 1)
				| match c {
					'{' => 0,
					'}' => 1 << 7,
					_ => unreachable!(),
				};
			bit_count += 1;
			if bit_count > 7 {
				bit_count = 0;
				bytes.push(current_byte);
				current_byte = 0;
			}
		}
		if bit_count != 0 {
			bytes.push(current_byte);
		}
		bytes.reverse();
		bytes
	}
}

impl Display for SetVersion {
	/// The stringified version is always in canonical form, meaning that small sets are printed first.
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{")?;
		for version in &self.versions {
			version.fmt(f)?;
		}
		write!(f, "}}")
	}
}

impl From<&SetVersion> for String {
	fn from(this: &SetVersion) -> Self {
		format!("{}", this)
	}
}

impl From<&SetVersion> for u128 {
	fn from(this: &SetVersion) -> Self {
		let bytes = this.to_integralternative_bytes();
		// Not representable
		if bytes.len() > 128 / 8 {
			panic!("Integralternative of {} is too large to be represented with u128", this);
		}
		let mut result = 0u128;
		for byte in bytes {
			result = (result << 8) | (byte as u128);
		}
		result
	}
}

/// The errors that can happen when parsing a SetVer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SetVerParseError {
	/// An illegal character is in the parsed string. Stores the illegal character.
	IllegalCharacter(char),
	/// A set contains non-unique elements (sets).
	NonUniqueElements,
	/// A curly brace is unclosed.
	UnclosedBrace,
	/// The string is empty.
	Empty,
	/// There's more than one set here.
	TooManySets,
}

impl Display for SetVerParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match &self {
				Self::IllegalCharacter(c) => format!("Illegal character '{}'", c),
				Self::NonUniqueElements => "Set contains non-unique subsets".to_string(),
				Self::UnclosedBrace => "Unclosed set brace".to_string(),
				Self::Empty => "Empty string".to_string(),
				Self::TooManySets => "Too many sets (more than one)".to_string(),
			}
		)
	}
}

impl FromStr for SetVersion {
	type Err = SetVerParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		// The smallest allowed setver specification is "{}" at length 2.
		if value.len() < 2 {
			return Err(SetVerParseError::Empty);
		}
		let mut chars = value.chars();
		let open_curly = chars.next().unwrap();
		if open_curly != '{' {
			return Err(SetVerParseError::IllegalCharacter(open_curly));
		}

		// Find the matching brace.
		let mut brace_level = 1;
		let mut inner_sets = vec!["".to_owned()];
		for next_char in &mut chars {
			match next_char {
				'{' => brace_level += 1,
				'}' => brace_level -= 1,
				_ => return Err(SetVerParseError::IllegalCharacter(next_char)),
			}
			if brace_level == 0 {
				break;
			}
			inner_sets.last_mut().unwrap().push(next_char);
			if brace_level == 1 {
				inner_sets.push("".to_owned());
			}
		}
		if brace_level != 0 {
			return Err(SetVerParseError::UnclosedBrace);
		}
		if chars.next() != None {
			return Err(SetVerParseError::TooManySets);
		}

		// The last set is a still-empty character collector if we got braces to match correctly.
		inner_sets.remove(inner_sets.len() - 1);
		if inner_sets.is_empty() {
			return Ok(Self::default());
		}

		let versions = inner_sets
			.iter()
			.map(|string_set| string_set.parse())
			.collect::<Result<BTreeSet<SetVersion>, SetVerParseError>>()?;
		if versions.len() < inner_sets.len() {
			return Err(SetVerParseError::NonUniqueElements);
		}
		Ok(Self { versions })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_correct_setver() -> Result<(), SetVerParseError> {
		// Note that there are other valid forms of these SetVer versions, but we need to use the canonical form where the smallest sets come first so that we can ensure they re-serialize correctly.
		for test_string in
			["{}", "{{}}", "{{}{{}}}", "{{}{{}}{{}{{}}}}", "{{{{{{{}}}}}}}", "{{}{{}}{{{}}}}", "{{}{{{}}{{}{{}}}}}"]
		{
			assert_eq!(test_string.parse::<SetVersion>()?.to_string(), test_string);
		}

		Ok(())
	}

	#[test]
	fn parse_incorrect_setver() {
		assert_eq!("".parse::<SetVersion>().unwrap_err(), SetVerParseError::Empty);
		assert_eq!("asd".parse::<SetVersion>().unwrap_err(), SetVerParseError::IllegalCharacter('a'));
		assert_eq!("{{b}}".parse::<SetVersion>().unwrap_err(), SetVerParseError::IllegalCharacter('b'));
		"{{}{}".parse::<SetVersion>().unwrap_err();
		"}{}".parse::<SetVersion>().unwrap_err();
		assert_eq!("{}{}".parse::<SetVersion>().unwrap_err(), SetVerParseError::TooManySets);
		assert_eq!("{{}{}}".parse::<SetVersion>().unwrap_err(), SetVerParseError::NonUniqueElements);
		assert_eq!("{{{}{}}{}}".parse::<SetVersion>().unwrap_err(), SetVerParseError::NonUniqueElements);
		assert_eq!("{{}{{}{{}}}{{}{{}}}}".parse::<SetVersion>().unwrap_err(), SetVerParseError::NonUniqueElements);
	}

	#[test]
	fn equality() {
		assert_eq!("{}".parse::<SetVersion>().unwrap(), "{}".parse::<SetVersion>().unwrap());
		assert_ne!("{{{}}}".parse::<SetVersion>().unwrap(), "{{}}".parse::<SetVersion>().unwrap());
		assert_eq!("{{}{{}}}".parse::<SetVersion>().unwrap(), "{{{}}{}}".parse::<SetVersion>().unwrap());
		assert_eq!(
			"{{{{}{{}}}{{}}}{}}".parse::<SetVersion>().unwrap(),
			"{{}{{{}}{{}{{}}}}}".parse::<SetVersion>().unwrap()
		);
	}

	#[test]
	fn integralternative() {
		assert_eq!("{{}{{{}}{{}{{}}}}}".parse::<SetVersion>().unwrap().to_integralternative(), 35999);
	}
}
