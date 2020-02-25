//! Character test functions.

use super::constants::*;
use super::util::char_in_range;

// TODO: '゠' | '＝' | '・'

/// Returns true for non-kana phonetic marks used intra word.
pub fn is_word_mark(chr: char) -> bool {
	match chr {
		// Katakana and Hiragana Iteration marks
		'ヽ' | 'ヾ' | 'ゝ' | 'ゞ' => true,
		// Dakuten and Handakuten
		'゛' | '゜' => true,
		_ => false,
	}
}

/// Returns true if the character is Hiragana or `ー`.
///
/// Note that this excludes characters from the hiragana block such as the
/// combining diacritics and marks from U+3099 and U+309F.
pub fn is_hiragana(chr: char) -> bool {
	match chr {
		'ゟ' | 'ー' => true, // U+309F - Hiragana Digraph Yori
		_ => char_in_range(chr, HIRAGANA_START, HIRAGANA_END),
	}
}

/// Returns true if the character is Katakana or `ー`.
pub fn is_katakana(chr: char) -> bool {
	match chr {
		'ヿ' | 'ー' => true, // U+30FF - Katakana Digraph Koto
		_ => char_in_range(chr, KATAKANA_START, KATAKANA_END),
	}
}

/// Returns true if the character is a kanji.
pub fn is_kanji(chr: char) -> bool {
	char_in_range(chr, KANJI_START, KANJI_END)
}

/// Returns true if the character is hiragana or katakana.
pub fn is_kana(chr: char) -> bool {
	is_hiragana(chr) || is_katakana(chr)
}

/// Returns true if the character is a japanese-style punctuation.
pub fn is_japanese_punctuation(chr: char) -> bool {
	match chr as u32 {
		// CJK Symbols and Punctuation
		0x3000..=0x303F => true,

		// Katakana punctuation
		0x30FB => true,

		// Kana punctuation
		0xFF61..=0xFF65 => true, // `｡` to `･`

		// Zenkaku punctuation (Halfwidth and Fullwidth Forms)
		0xFF01..=0xFF0F => true,              // `！` to `／`
		0xFF1A..=0xFF1F => true,              // `：` to `？`
		0xFF3B..=0xFF3F => chr != '\u{FF3E}', // `［` to `＿`, but not `＾`
		0xFF5B..=0xFF60 => true,              // `｛` to `｠`

		// Currency symbols
		0xFFE0..=0xFFEE => true,

		_ => false,
	}
}
