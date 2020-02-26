//! Kind of japanese characters.

use super::constants::*;

/// Enumeration with character kinds.
#[derive(Debug, PartialEq)]
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

// spell-checker: disable

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_kind() {
		// Sources:
		// - https://stackoverflow.com/questions/19899554/unicode-range-for-japanese/19945665
		// - https://japanese.stackexchange.com/questions/27393/what-phonetic-shorthands-like-%E3%80%BC-%E3%80%86-are-there-in-japanese
		// - https://en.wikipedia.org/wiki/List_of_Japanese_typographic_symbols
		// - http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
		// - https://www.compart.com/en/unicode/ (lookup)
		// - https://unicode-table.com/ (lookup)

		const BAR_LINE: &'static str = "ーｰ";
		const HIRAGANA: &'static str = concat!(
			"ぁあぃいぅうぇえぉお",
			"かがきぎくぐけげこご",
			"さざしじすずせぜそぞ",
			"ただちぢっつづてでとど",
			"なにぬねの",
			"はばぱひびぴふぶぷへべぺほぼぽ",
			"まみむめも",
			"ゃやゅゆょよ",
			"らりるれろ",
			"ゎわゐゑをんゔゕゖゟ",
		);
		const KATAKANA: &'static str = concat!(
			"ァアィイゥウェエォオ",
			"カガキギクグケゲコゴ",
			"サザシジスズセゼソゾ",
			"タダチヂッツヅテデトド",
			"ナニヌネノ",
			"ハバパヒビピフブプヘベペホボポ",
			"マミムメモ",
			"ャヤュユョヨ",
			"ラリルレロ",
			"ヮワヰヱヲンヴヵヶヷヸヹヺ",
			"ヿ",
			"ㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ",
		);
		const KATAKANA_HALF: &'static str = concat!(
			"ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ",
			"ｦｧｨｩｪｫｬｭｮｯ"
		);
		const ROMAJI: &'static str = concat!(
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
			"abcdefghijklmnopqrstuvwxyz",
			"01234567890",
			"āīūēōâîûêô",
		);
		const ROMAN_DIGITS: &'static str = "０１２３４５６７８９";
		const ROMAN_LETTERS: &'static str = concat!(
			"ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ",
			"ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ",
		);
		const ROMAN_PUNCTUATION: &'static str =
			concat!("！＂＃＄％＆＇（）＊＋，－．／：；＜＝＞？＠［＼］＾＿｀｛｜｝～");

		const JAPANESE_PUNCTUATION: &'static str = concat!(
			"゠・",
			"　、。〃〈〉《》「」『』【】〔〕〖〗〘〙〚〛〜〝〞〟〰〽",
			"｟｠｡｢｣､･",
		);
		const JAPANESE_MARK: &'static str = concat!("゛゜ゝゞヽヾ", "々〆〱〲〳〴〵〻〼",);
		const JAPANESE_SYMBOL: &'static str = concat!(
			"〄〇〒〠〶〷〾〿〓￠￮",
			"㈠㈡㈢㈣㈤㈥㈦㈧㈨㈩㈪㈫㈬㈭㈮㈯㈰㈱㈲㈳㈴㈵㈶㈷㈸㈹㈺㈻㈼㈽㈾㈿㉀㉁㉂㉃㊀㊁㊂㊃",
			"㊄㊅㊆㊇㊈㊉㊊㊋㊌㊍㊎㊏㊐㊑㊒㊓㊔㊕㊖㊗㊘㊙㊚㊛㊜㊝㊞㊟㊠㊡㊢㊣㊤㊥㊦㊧㊨㊩㊪㊫",
			"㊬㊭㊮㊯㊰㊱㊲㊳㊴㊵㊶㊷㊸㊹㊺㊻㊼㊽㊾㊿㋀㋁㋂㋃㋄㋅㋆㋇㋈㋉㋊㋋㋐㋑㋒㋓㋔㋕㋖㋗",
			"㋘㋙㋚㋛㋜㋝㋞㋟㋠㋡㋢㋣㋤㋥㋦㋧㋨㋩㋪㋫㋬㋭㋮㋯㋰㋱㋲㋳㋴㋵㋶㋷㋸㋹㋺㋻㋼㋽㋾㌀",
			"㌁㌂㌃㌄㌅㌆㌇㌈㌉㌊㌋㌌㌍㌎㌏㌐㌑㌒㌓㌔㌕㌖㌗㌘㌙㌚㌛㌜㌝㌞㌟㌠㌡㌢㌣㌤㌥㌦㌧㌨",
			"㌩㌪㌫㌬㌭㌮㌯㌰㌱㌲㌳㌴㌵㌶㌷㌸㌹㌺㌻㌼㌽㌾㌿㍀㍁㍂㍃㍄㍅㍆㍇㍈㍉㍊㍋㍌㍍㍎㍏㍐",
			"㍑㍒㍓㍔㍕㍖㍗㍘㍙㍚㍛㍜㍝㍞㍟㍠㍡㍢㍣㍤㍥㍦㍧㍨㍩㍪㍫㍬㍭㍮㍯㍰㍱㍲㍳㍴㍵㍶㍻㍼",
			"㍽㍾㍿㎀㎁㎂㎃㎄㎅㎆㎇㎈㎉㎊㎋㎌㎍㎎㎏㎐㎑㎒㎓㎔㎕㎖㎗㎘㎙㎚㎛㎜㎝㎞㎟㎠㎡㎢㎣㎤",
			"㎥㎦㎧㎨㎩㎪㎫㎬㎭㎮㎯㎰㎱㎲㎳㎴㎵㎶㎷㎸㎹㎺㎻㎼㎽㎾㎿㏀㏁㏂㏃㏄㏅㏆㏇㏈㏉㏊㏋㏌",
			"㏍㏎㏏㏐㏑㏒㏓㏔㏕㏖㏗㏘㏙㏚㏛㏜㏝㏞㏟㏠㏡㏢㏣㏤㏥㏦㏧㏨㏩㏪㏫㏬㏭㏮㏯㏰㏱㏲㏳㏴",
			"㏵㏶㏷㏸㏹㏺㏻㏼㏽㏾㏿",
			// Kanji radicals
			"⺀⺁⺂⺃⺄⺅⺆⺇⺈⺉⺊⺋⺌⺍⺎⺏⺐⺑⺒⺓⺔⺕⺖⺗⺘⺙⺚⺛⺜⺝⺞⺟⺠⺡⺢⺣⺤⺥⺦⺧",
			"⺨⺩⺪⺫⺬⺭⺮⺯⺰⺱⺲⺳⺴⺵⺶⺷⺸⺹⺺⺻⺼⺽⺾⺿⻀⻁⻂⻃⻄⻅⻆⻇⻈⻉⻊⻋⻌⻍⻎⻏",
			"⻐⻑⻒⻓⻔⻕⻖⻗⻘⻙⻚⻛⻜⻝⻞⻟⻠⻡⻢⻣⻤⻥⻦⻧⻨⻩⻪⻫⻬⻭⻮⻯⻰⻱⻲⻳⼀⼁⼂⼃",
			"⼄⼅⼆⼇⼈⼉⼊⼋⼌⼍⼎⼏⼐⼑⼒⼓⼔⼕⼖⼗⼘⼙⼚⼛⼜⼝⼞⼟⼠⼡⼢⼣⼤⼥⼦⼧⼨⼩⼪⼫",
			"⼬⼭⼮⼯⼰⼱⼲⼳⼴⼵⼶⼷⼸⼹⼺⼻⼼⼽⼾⼿⽀⽁⽂⽃⽄⽅⽆⽇⽈⽉⽊⽋⽌⽍⽎⽏⽐⽑⽒⽓",
			"⽔⽕⽖⽗⽘⽙⽚⽛⽜⽝⽞⽟⽠⽡⽢⽣⽤⽥⽦⽧⽨⽩⽪⽫⽬⽭⽮⽯⽰⽱⽲⽳⽴⽵⽶⽷⽸⽹⽺⽻",
			"⽼⽽⽾⽿⾀⾁⾂⾃⾄⾅⾆⾇⾈⾉⾊⾋⾌⾍⾎⾏⾐⾑⾒⾓⾔⾕⾖⾗⾘⾙⾚⾛⾜⾝⾞⾟⾠⾡⾢⾣",
			"⾤⾥⾦⾧⾨⾩⾪⾫⾬⾭⾮⾯⾰⾱⾲⾳⾴⾵⾶⾷⾸⾹⾺⾻⾼⾽⾾⾿⿀⿁⿂⿃⿄⿅⿆⿇⿈⿉⿊⿋",
			"⿌⿍⿎⿏⿐⿑⿒⿓⿔⿕",
		);

		const PUNCTUATION_ASCII: &'static str = " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
		const KANJI: &'static str = concat!(
			// Random assortment of kanji
			"漢字日本語文字言語言葉",
			"一丁丂七丄丅丆万丈三上下丌不与丏",
			"捰捱捲捳捴捵捶捷捸捹捺捻捼捽捾捿",
			"満溁溂溃溄溅溆溇溈溉溊溋溌溍溎溏",
			"觐觑角觓觔觕觖觗觘觙觚觛觜觝觞觟",
			"䁰䁱䁲䁳䁴䁵䁶䁷䁸䁹䁺䁻䁼䁽䁾䁿",
			"䰀䰁䰂䰃䰄䰅䰆䰇䰈䰉䰊䰋䰌䰍䰎䰏",
			"䶰䶱䶲䶳䶴䶵",
			// Extension A
			"㐀䰼䰽䰾䩍䩎䩏䰿䶵",
			// Extension B
			"𠀀𠂹𠂺𠂻𠂼𠂽𠳜𠳝𠳞",
			"𪏲𪏴𪏵𪏶𩺔𩺕𩺗𩺘𪛖",
			// Extension C
			"𪜀𫙑𫙒𫙓𫑘𫑙𫑚𫑝𫜴",
			// Extension D
			"𫝀𫞁𫞂𫞃𫞄𫟅𫟇𫟉𫠝",
		);
		const NONE: &'static str = concat!("〡〢〣〤〥〦〧〨〩〸〹〺ãç");

		check(BAR_LINE, CharKind::BarLine);

		check(HIRAGANA, CharKind::Hiragana);
		check(KATAKANA, CharKind::Katakana);
		check(ROMAJI, CharKind::Romaji);
		check(KANJI, CharKind::Kanji);

		check(KATAKANA_HALF, CharKind::KatakanaHalfWidth);

		check(ROMAN_DIGITS, CharKind::RomanDigit);
		check(ROMAN_LETTERS, CharKind::RomanLetter);
		check(ROMAN_PUNCTUATION, CharKind::RomanPunctuation);

		check(JAPANESE_PUNCTUATION, CharKind::JapanesePunctuation);
		check(JAPANESE_MARK, CharKind::JapaneseMark);
		check(JAPANESE_SYMBOL, CharKind::JapaneseSymbol);

		check(PUNCTUATION_ASCII, CharKind::PunctuationASCII);

		check(NONE, CharKind::None);

		fn check(input: &'static str, expected: CharKind) {
			for chr in input.chars() {
				let kind = get_kind(chr);
				assert_eq!(
					kind, expected,
					"expected kind of `{}` (U+{:04X}) to be {:?}, but it was {:?}",
					chr, chr as u32, expected, kind,
				);
			}
		}
	}
}
