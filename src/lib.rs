//! SetVer comprehension for Rust.

#![deny(missing_docs, clippy::all)]

use std::collections::BTreeSet;
use std::fmt::Display;

/// A SetVer version specification.
/// # Implementation details
/// This struct is implemented using HashSet from the standard library.
/// Therefore, it is not usable in no-std environments right now.
#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd, Default)]
pub struct SetVersion {
	/// Making this an ordered set guarantees that all iterations are performed in order, which gives some nice guarantees for faster implementations.
	versions: BTreeSet<SetVersion>,
}

impl Display for SetVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{")?;
		for version in &self.versions {
			version.fmt(f)?;
		}
		write!(f, "}}")
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

impl TryFrom<&str> for SetVersion {
	type Error = SetVerParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		// The smallest allowed setver specification is "{}" at length 2.
		if value.len() < 2 {
			return Err(SetVerParseError::Empty);
		}
		let mut chars = value.chars();
		let open_curly = chars.nth(0).unwrap();
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
		if inner_sets.len() == 0 {
			return Ok(Self::default());
		}

		let versions = inner_sets
			.iter()
			.map(|string_set| string_set.as_str().try_into())
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
			assert_eq!(SetVersion::try_from(test_string)?.to_string(), test_string);
		}

		Ok(())
	}

	#[test]
	fn parse_incorrect_setver() {
		assert_eq!(SetVersion::try_from("").unwrap_err(), SetVerParseError::Empty);
		assert_eq!(SetVersion::try_from("asd").unwrap_err(), SetVerParseError::IllegalCharacter('a'));
		assert_eq!(SetVersion::try_from("{{b}}").unwrap_err(), SetVerParseError::IllegalCharacter('b'));
		SetVersion::try_from("{{}{}").unwrap_err();
		SetVersion::try_from("}{}").unwrap_err();
		assert_eq!(SetVersion::try_from("{}{}").unwrap_err(), SetVerParseError::TooManySets);
		assert_eq!(SetVersion::try_from("{{}{}}").unwrap_err(), SetVerParseError::NonUniqueElements);
		assert_eq!(SetVersion::try_from("{{{}{}}{}}").unwrap_err(), SetVerParseError::NonUniqueElements);
		assert_eq!(SetVersion::try_from("{{}{{}{{}}}{{}{{}}}}").unwrap_err(), SetVerParseError::NonUniqueElements);
	}
}
