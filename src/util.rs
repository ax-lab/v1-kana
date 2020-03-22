//! Internal utility functions.

/// Check if the character is in the given range (inclusive).
#[inline]
pub fn char_in_range(c: char, start: u32, end: u32) -> bool {
	let c = c as u32;
	c >= start && c <= end
}

/// Return a prefix of at most `n` characters for the given string.
#[inline]
pub fn get_prefix(s: &str, n: usize) -> &str {
	let end = s.char_indices().map(|x| x.0).nth(n).unwrap_or(s.len());
	&s[..end]
}

/// Returns true if the character is a Romaji consonant.
#[inline]
pub fn is_consonant(c: char, include_y: bool) -> bool {
	match c {
		'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' => true,
		'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' => true,
		'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'z' => true,
		'N' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'V' | 'W' | 'X' | 'Z' => true,
		'y' | 'Y' => include_y,
		_ => false,
	}
}

/// Simple conversion of Hiragana to Katakana. Unknown characters just pass
/// through.
#[inline]
pub fn hiragana_to_katakana(c: char) -> char {
	use super::constants::*;

	const OFFSET: u32 = KATAKANA_TO_HIRAGANA_OFFSET_SUB;
	const RANGE_START: u32 = KATAKANA_START - OFFSET;
	const RANGE_END: u32 = KATAKANA_TO_HIRAGANA_END - OFFSET;

	if char_in_range(c, RANGE_START, RANGE_END) {
		let code = (c as u32) + OFFSET;
		unsafe { std::char::from_u32_unchecked(code) }
	} else {
		match c {
			'ゝ' => 'ヽ',
			'ゞ' => 'ヾ',
			_ => c,
		}
	}
}

/// Converts a romaji syllable to the voiced equivalent.
pub fn romaji_to_voiced(input: &str) -> &'static str {
	match input {
		"ka" => "ga",
		"ki" => "gi",
		"ku" => "gu",
		"ke" => "ge",
		"ko" => "go",

		"sa" => "za",
		"shi" => "ji",
		"su" => "zu",
		"se" => "ze",
		"so" => "zo",

		"ta" => "da",
		"chi" => "di",
		"tsu" => "du",
		"te" => "de",
		"to" => "do",

		"ha" => "ba",
		"hi" => "bi",
		"fu" => "bu",
		"he" => "be",
		"ho" => "bo",

		_ => "",
	}
}
