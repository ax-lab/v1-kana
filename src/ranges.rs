/// Pattern for the prolonged sound mark and its halfwidth version.
macro_rules! prolonged_mark_range {
	() => {
		// U+30FC  "ー"  Katakana-Hiragana Prolonged Sound Mark
		// U+FF70  "ｰ"   Halfwidth Katakana-Hiragana Prolonged Sound Mark
		('ー' | 'ｰ')
	};
}

/// Pattern for Hiragana letters, including small letters.
macro_rules! hiragana_range {
	() => {
		// U+309F   "ゟ"  Hiragana Digraph Yori
		// U+1B001  "𛀁"  Hiragana Letter Archaic Ye
		// U+3041   "ぁ"  Hiragana Letter Small A
		// U+3096   "ゖ"  Hiragana Letter Small Ke
		('ゟ' | '𛀁' | '\u{3041}'..='\u{3096}')
	};
}

/// Pattern for Katakana letters, including small letters.
///
/// See also `katakana_half_range`.
macro_rules! katakana_range {
	() => {
		// U+30FF  "ヿ"  Katakana Digraph Koto
		// U+30A1  "ァ"  Katakana Letter Small A
		// U+30FA  "ヺ"  Katakana Letter Vo
		// U+31F0  "ㇰ"  Katakana Letter Small Ku
		// U+31FF  "ㇿ"  Katakana Letter Small Ro
		('ヿ' | '\u{30A1}'..='\u{30FA}' | '\u{31F0}'..='\u{31FF}')
	};
}

/// Pattern for halfwidth Katakana letters.
macro_rules! katakana_half_range {
	() => {
		// U+FF66  "ｦ"  Halfwidth Katakana Letter Wo
		// U+FF6F  "ｯ"  Halfwidth Katakana Letter Small Tu
		// U+FF70  "ｰ"  Halfwidth Katakana-Hiragana Prolonged Sound Mark (skipped)
		// U+FF71  "ｱ"  Halfwidth Katakana Letter A
		// U+FF9D  "ﾝ"  Halfwidth Katakana Letter N
		('\u{FF66}'..='\u{FF6F}' | '\u{FF71}'..='\u{FF9D}')
	};
}

/// Pattern for romaji letters A to Z (including `Â..Û` and `Ā...Ū`) and
/// digits `0` to `9`.
macro_rules! romaji_range {
	() => {
		(
			'A'..='Z' | 'a'..='z' | '0'..='9'
			| 'â' | 'ê' | 'î' | 'ô' | 'û'
			| 'Â' | 'Ê' | 'Î' | 'Ô' | 'Û'
			| 'ā' | 'ē' | 'ī' | 'ō' | 'ū'
			| 'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū'
		)
	};
}

/// Pattern for Kanji ranges. Note that this includes Kanji from all languages
/// not limited to japanese.
///
/// This includes all Kanji from the "CJK Unified Ideographs" and the
/// extensions blocks from A to F.
///
macro_rules! kanji_range {
	() => {
		(
			// CJK Unified Ideographs (`一` to `龯`)
			'\u{4E00}'..='\u{9FAF}'

			// Rare and uncommon kanji extensions:

			// CJK Unified Ideographs Extension A (`㐀` to `䶵`)
			| '\u{3400}'..='\u{4DB5}'
			// CJK Unified Ideographs Extension B (`𠀀` to `𪛖`)
			| '\u{20000}'..='\u{2A6D6}'
			// CJK Unified Ideographs Extension C (`𪜀` to `𫜴`)
			| '\u{2A700}'..='\u{2B734}'
			// CJK Unified Ideographs Extension D (`𫝀` to `𫠝`)
			| '\u{2B740}'..='\u{2B81D}'
			// CJK Unified Ideographs Extension E (most don't render)
			| '\u{2B820}'..='\u{2CEAF}'
			// CJK Unified Ideographs Extension F (none renders)
			| '\u{2CEB0}'..='\u{2EBEF}'
		)
	};
}

/// Pattern for ASCII punctuation.
macro_rules! ascii_punctuation_range {
	() => {
		(
			' ' | '`' | '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*'
			| '(' | ')' | '-' | '_' | '=' | '+' | '[' | ']' | '{' | '}' | ';'
			| ':' | '<' | '>' | ',' | '.' | '/' | '?' | '\'' | '"' | '|' | '\\'
		)
	};
}

/// Pattern for fullwidth roman digits `０` to `９`.
macro_rules! roman_digit_range {
	() => {
		('０'..='９')
	};
}

/// Pattern for fullwidth roman letters `Ａ` to `Ｚ` and `ａ` to `ｚ`.
macro_rules! roman_letter_range {
	() => {
		('Ａ'..='Ｚ' | 'ａ'..='ｚ')
	};
}

/// Pattern for fullwidth roman punctuation characters.
macro_rules! roman_punctuation_range {
	() => {
		('！'..='／' | '：'..='＠' | '［'..='｀' | '｛'..='～')
	};
}

/// Pattern for japanese punctuation characters, including space.
macro_rules! japanese_punctuation_range {
	() => {
		(
			// U+3000 Ideographic Space
			'　'

			// Most of these are from "CJK Symbols and Punctuation"
			| '｟'..='･'
			| '、' | '。' | '〃' | '〈' | '〉' | '《' | '》' | '「' | '」' | '『'
			| '』' | '【' | '】' | '〔' | '〕' | '〖' | '〗' | '〘' | '〙' | '〚'
			| '〛' | '〜' | '〝' | '〞' | '〟' | '〰' | '〽' | '゠' | '・'
		)
	};
}

/// Pattern for japanese mark characters. The difference between the mark
/// characters and general symbols are that the mark characters have a direct
/// effect on the transliteration.
macro_rules! japanese_mark_range {
	() => {
		(
			'々' | '〆' | '〱' | '〲' | '〳' | '〴' | '〵' | '〻' | '〼' | '゛'
			| '゜' | 'ゝ' | 'ゞ' | 'ヽ' | 'ヾ'
		)
	};
}

/// Pattern for general japanese symbols.
macro_rules! japanese_symbol_range {
	() => {
		(
			'〄' | '〇' | '〒' | '〓' | '〠' | '〶' | '〷' | '〾' | '〿'
			| '￠'..='￮' | '㈀'..='㋾' | '㌀'..='㏿' | '⺀'..='⻳' | '⼀'..='⿕'
		)
	};
}
