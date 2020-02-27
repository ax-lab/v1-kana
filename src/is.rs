//! Character test functions.

/// Returns true if the character is Hiragana letter.
///
/// Note that this excludes characters from the hiragana block such as the
/// combining diacritics and marks from U+3099 and U+309F.
pub fn is_hiragana(chr: char) -> bool {
	match chr {
		hiragana_range!() => true,
		_ => false,
	}
}

/// Returns true if the character is a Katakana letter.
pub fn is_katakana(chr: char) -> bool {
	match chr {
		katakana_range!() => true,
		katakana_half_range!() => true,
		_ => false,
	}
}

/// Returns true if the character is a Kanji letter.
pub fn is_kanji(chr: char) -> bool {
	match chr {
		kanji_range!() => true,
		_ => false,
	}
}

/// Returns true if the character is hiragana, katakana or the prolonged sound
/// mark.
pub fn is_kana(chr: char) -> bool {
	match chr {
		prolonged_mark_range!() => true,
		hiragana_range!() => true,
		katakana_range!() => true,
		katakana_half_range!() => true,
		_ => false,
	}
}

/// Returns true if the character is hiragana, katakana, kanji or the prolonged
/// sound mark.
pub fn is_letter(chr: char) -> bool {
	is_kana(chr) || is_kanji(chr)
}

/// Returns true for Japanese word marks, including the prolonged sound mark.
pub fn is_japanese_mark(chr: char) -> bool {
	match chr {
		prolonged_mark_range!() => true,
		japanese_mark_range!() => true,
		_ => false,
	}
}

/// Returns true if the character is a japanese-style punctuation.
pub fn is_japanese_punctuation(chr: char) -> bool {
	match chr {
		japanese_punctuation_range!() => true,
		_ => false,
	}
}

// spell-checker: disable

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_hiragana() {
		let s = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖゐゑゟ";
		for chr in s.chars() {
			assert!(is_hiragana(chr), "is_hiragana({})", chr);
		}

		for code in 0x3041..=0x3096 {
			let chr = std::char::from_u32(code).unwrap();
			assert!(is_hiragana(chr), "is_hiragana(U+{:04X})", code);
		}

		for chr in "ｰー゠・".chars() {
			assert!(!is_hiragana(chr), "!is_hiragana({})", chr);
		}

		assert!(!is_hiragana('\u{3040}'));
		assert!(!is_hiragana('\u{3097}'));
	}

	#[test]
	fn test_is_katakana() {
		let s = "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヷヸヹヺヿ";
		for chr in s.chars() {
			assert!(is_katakana(chr), "is_katakana({})", chr);
		}

		for code in 0x30A1..=0x30FA {
			let chr = std::char::from_u32(code).unwrap();
			assert!(is_katakana(chr), "is_katakana(U+{:04X})", code);
		}

		for chr in "ｰー゠・".chars() {
			assert!(!is_katakana(chr), "!is_katakana({})", chr);
		}

		assert!(!is_katakana('\u{30A0}'));
		assert!(!is_katakana('\u{30FB}'));
	}

	#[test]
	fn test_is_kanji() {
		let s = "一切腹刀丁丂七丄丅丆万丈三上下丌不与丏岐岑岒岓岔岕岖岗岘岙岚岛岜岝岞岟棰棱棲棳棴棵棶棷棸棹棺棻棼棽棾棿龠龡龢龣龤龥龦龧龨龩龪龫龬龭龮龯";
		for chr in s.chars() {
			assert!(is_kanji(chr), "is_kanji({}) -- 0x{:04X}", chr, chr as u32);
		}

		for code in 0x4E00..=0x9FAF {
			let chr = std::char::from_u32(code).unwrap();
			assert!(is_kanji(chr), "is_kanji(U+{:04X})", code);
		}

		assert!(!is_kanji('\u{4DFF}'));
		assert!(!is_kanji('\u{9FB0}'));
	}
}
