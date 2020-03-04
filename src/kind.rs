//! Kind of japanese characters.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Enumeration with character kinds.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharKind {
	/// Any character that is neither japanese nor romaji.
	None,

	/// Full-width hiragana characters.
	///
	/// Note that as opposed to `is_hiragana`, this does not include `ー`.
	Hiragana,

	/// Full-width katakana characters.
	///
	/// Note that as opposed to `is_katakana`, this does not include `ー`.
	Katakana,

	/// Half width katakana characters.
	KatakanaHalfWidth,

	/// Kanji characters.
	Kanji,

	/// The prolonged sound mark characters (`ー` U+30FC and `ｰ` U+FF70 halfwidth).
	BarLine,

	/// Japanese punctuation marks, i.e. characters that split words and phrases.
	///
	/// This includes U+3000 Ideographic Space. This does NOT include the
	/// prolonged sound mark (see `BarLine`).
	///
	/// Examples of this are `、`, `。`, `・`, `〖`, `〗`, `「`, `」`.
	///
	/// See also `RomanFullWidthPunctuation`.
	JapanesePunctuation,

	/// Japanese repetition and iteration marks. Those are usually modifiers
	/// that affect transliteration.
	///
	/// Examples of this are `々`, `ヽ`, `ヾ`, `ゝ`, `ゞ`, `〱`, `〲`
	JapaneseMark,

	/// Japanese symbols that do not belong to a specific category.
	JapaneseSymbol,

	/// Full width roman digit from `０` to `９`.
	RomanDigit,

	/// Full width roman characters (`Ａ-Ｚ` and `ａ-ｚ`).
	RomanLetter,

	/// Full-width and half-width roman punctuation (e.g. `：`, `；`, `＜`, `＝`, `＞`).
	RomanPunctuation,

	/// ASCII punctuation characters, including spaces.
	PunctuationASCII,

	/// Letters from `A-Z` and digits `0-9`.
	Romaji,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_kind(chr: char) -> CharKind {
	match chr {
		prolonged_mark_range!() => CharKind::BarLine,
		hiragana_range!() => CharKind::Hiragana,
		katakana_range!() => CharKind::Katakana,
		katakana_half_range!() => CharKind::KatakanaHalfWidth,
		romaji_range!() => CharKind::Romaji,
		kanji_range!() => CharKind::Kanji,
		ascii_punctuation_range!() => CharKind::PunctuationASCII,
		roman_digit_range!() => CharKind::RomanDigit,
		roman_letter_range!() => CharKind::RomanLetter,
		roman_punctuation_range!() => CharKind::RomanPunctuation,
		japanese_punctuation_range!() => CharKind::JapanesePunctuation,
		japanese_mark_range!() => CharKind::JapaneseMark,
		japanese_symbol_range!() => CharKind::JapaneseSymbol,
		_ => CharKind::None,
	}
}
