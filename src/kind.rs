//! Kind of japanese characters.

use super::constants::*;

/// Enumeration with character kinds.
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

pub fn get_kind(chr: char) -> CharKind {
	match chr {
		//
		// Latin range
		//
		'A'..='Z' | 'a'..='z' => CharKind::Romaji,
		'â' | 'ê' | 'î' | 'ô' | 'û' => CharKind::Romaji,
		'Â' | 'Ê' | 'Î' | 'Ô' | 'Û' => CharKind::Romaji,
		'ā' | 'ē' | 'ī' | 'ō' | 'ū' => CharKind::Romaji,
		'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū' => CharKind::Romaji,
		'0'..='9' => CharKind::Romaji,
		' ' | '`' | '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '_'
		| '=' | '+' | '[' | ']' | '{' | '}' | ';' | ':' | '<' | '>' | ',' | '.' | '/' | '?'
		| '\'' | '"' | '|' | '\\' => CharKind::PunctuationASCII,

		//
		// Kana
		//
		'ー' | 'ｰ' => CharKind::BarLine,
		'ゟ' => CharKind::Hiragana, // U+309F Hiragana Digraph Yori
		'ヿ' => CharKind::Katakana, // U+30FF Katakana Digraph Koto

		//
		// Roman full and halfwidth
		//
		'０'..='９' => CharKind::RomanDigit,
		'Ａ'..='Ｚ' => CharKind::RomanLetter,
		'ａ'..='ｚ' => CharKind::RomanLetter,
		'！'..='／' => CharKind::RomanPunctuation,
		'：'..='＠' => CharKind::RomanPunctuation,
		'［'..='｀' => CharKind::RomanPunctuation,
		'｛'..='～' => CharKind::RomanPunctuation,

		//
		// Japanese punctuation
		//

		// CJK Symbols and Punctuation
		'　' | '、' | '。' | '〃' | '〈' | '〉' | '《' | '》' | '「' | '」' | '『' | '』'
		| '【' | '】' | '〔' | '〕' | '〖' | '〗' | '〘' | '〙' | '〚' | '〛' | '〜' | '〝'
		| '〞' | '〟' | '〰' | '〽' => CharKind::JapanesePunctuation,

		'々' | '〆' | '〱' | '〲' | '〳' | '〴' | '〵' | '〻' | '〼' | '゛' | '゜' | 'ゝ'
		| 'ゞ' => CharKind::JapaneseMark,

		'〄' | '〇' | '〒' | '〓' | '〠' | '〶' | '〷' | '〾' | '〿' => {
			CharKind::JapaneseSymbol
		}

		// Full and half-width punctuation (includes Kana)
		'｟'..='･' => CharKind::JapanesePunctuation,

		// Katakana
		'゠' | '・' => CharKind::JapanesePunctuation,
		'ヽ' | 'ヾ' => CharKind::JapaneseMark,

		// Full and halfwidth symbols (`￠` to `￮`)
		'￠'..='￮' => CharKind::JapaneseSymbol,

		//
		// Misc symbols
		//

		// Enclosed CJK Letters and Months
		'㈀'..='㋾' => CharKind::JapaneseSymbol,
		// CJK Compatibility
		'㌀'..='㏿' => CharKind::JapaneseSymbol,
		// CJK Radicals Supplement
		'⺀'..='⻳' => CharKind::JapaneseSymbol,
		// Kangxi Radicals
		'⼀'..='⿕' => CharKind::JapaneseSymbol,

		//
		// Numeric ranges
		//
		_ => match chr as u32 {
			//
			// Kana
			//
			HIRAGANA_START..=HIRAGANA_END => CharKind::Hiragana,
			KATAKANA_START..=KATAKANA_END => CharKind::Katakana,
			SMALL_KATAKANA_START..=SMALL_KATAKANA_END => CharKind::Katakana,
			HALF_KATAKANA_START..=HALF_KATAKANA_END => CharKind::KatakanaHalfWidth,

			//
			// Kanji
			//
			KANJI_START..=KANJI_END => CharKind::Kanji,
			KANJI_START_A..=KANJI_END_A => CharKind::Kanji,
			KANJI_START_B..=KANJI_END_B => CharKind::Kanji,
			KANJI_START_C..=KANJI_END_C => CharKind::Kanji,
			KANJI_START_D..=KANJI_END_D => CharKind::Kanji,

			//
			// Symbols
			//
			_ => CharKind::None,
		},
	}
}
