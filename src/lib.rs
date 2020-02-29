//! Japanese character (kana and kanji) utilities.
//!
//! This library provides fast translation between Hiragana, Katakana and Romaji
//! as well as utility functions to test different Japanese characters.

// Need this because of `ranges.rs`
#![allow(unused_parens)]
#![feature(or_patterns)]
// Need this for benchmarks
#![feature(test)]

extern crate fnv;
extern crate test;

#[macro_use]
extern crate lazy_static;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

// CharCode References
// http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
// http://unicode-table.com

#[cfg(target_arch = "wasm32")]
mod wasm;

mod constants;
mod table;
mod util;

#[macro_use]
mod ranges;

mod is;
pub use is::*;

mod to;
pub use to::*;

mod kind;
pub use kind::*;

// spell-checker: disable

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	const INPUT: &'static str = "
		ぁあぃいぅうぇえぉおかがきぎくぐけげこご
		ハバパヒビピフブプヘベペホボポ
		ABCDEFGHIJKLMNOPQRSTUVWXYZ
		ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ
		㈠㈡㈢㈣㈤㈥㈦㈧㈨㈩㈪㈫㈬㈭㈮㈯㈰㈱㈲㈳㈴㈵㈶㈷㈸㈹㈺㈻㈼㈽㈾㈿㉀㉁㉂㉃㊀㊁㊂㊃
		㊄㊅㊆㊇㊈㊉㊊㊋㊌㊍㊎㊏㊐㊑㊒㊓㊔㊕㊖㊗㊘㊙㊚㊛㊜㊝㊞㊟㊠㊡㊢㊣㊤㊥㊦㊧㊨㊩㊪㊫
		㊬㊭㊮㊯㊰㊱㊲㊳㊴㊵㊶㊷㊸㊹㊺㊻㊼㊽㊾㊿㋀㋁㋂㋃㋄㋅㋆㋇㋈㋉㋊㋋㋐㋑㋒㋓㋔㋕㋖㋗
		㋘㋙㋚㋛㋜㋝㋞㋟㋠㋡㋢㋣㋤㋥㋦㋧㋨㋩㋪㋫㋬㋭㋮㋯㋰㋱㋲㋳㋴㋵㋶㋷㋸㋹㋺㋻㋼㋽㋾㌀
		漢字日本語文字言語言葉
		一丁丂七丄丅丆万丈三上下丌不与丏
		捰捱捲捳捴捵捶捷捸捹捺捻捼捽捾捿
		満溁溂溃溄溅溆溇溈溉溊溋溌溍溎溏
		觐觑角觓觔觕觖觗觘觙觚觛觜觝觞觟
		䁰䁱䁲䁳䁴䁵䁶䁷䁸䁹䁺䁻䁼䁽䁾䁿
		䰀䰁䰂䰃䰄䰅䰆䰇䰈䰉䰊䰋䰌䰍䰎䰏
		䶰䶱䶲䶳䶴䶵㐀䰼䰽䰾䩍䩎䩏䰿䶵
		𠀀𠂹𠂺𠂻𠂼𠂽𠳜𠳝𠳞𪏲𪏴𪏵𪏶𩺔𩺕𩺗𩺘𪛖
		𪜀𫙑𫙒𫙓𫑘𫑙𫑚𫑝𫜴𫝀𫞁𫞂𫞃𫞄𫟅𫟇𫟉𫠝
	";

	#[bench]
	fn bench_is_kanji(b: &mut Bencher) {
		b.iter(|| {
			let mut count = 0;
			for chr in INPUT.chars() {
				if is_kanji(chr) {
					count += 1;
				}
			}
			count
		})
	}

	#[bench]
	fn bench_get_kind(b: &mut Bencher) {
		b.iter(|| {
			let mut count = 0;
			for chr in INPUT.chars() {
				if get_kind(chr) == CharKind::Kanji {
					count += 1;
				}
			}
			count
		})
	}

	#[test]
	fn test_char_kind() {
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
			// Extension E
			"\u{2B820}\u{2CEAF}𫢸𫢹𫭼𫭽𫮃𫮄𫰜𫰛𫸩𬀩𬀪𬃊",
			// Extension F
			"\u{2CEB0}\u{2EBEF}",
		);
		const NONE: &'static str = concat!("〡〢〣〤〥〦〧〨〩〸〹〺ãç");

		// Check the `CharKind` returned by `get_kind`:

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

		// Test the `is_xyz` functions:

		for chr in HIRAGANA.chars() {
			assert!(
				is_hiragana(chr),
				"expected `{}` (U+{:04X}) to be hiragana",
				chr,
				chr as u32,
			)
		}

		for chr in KATAKANA.chars() {
			assert!(
				is_katakana(chr),
				"expected `{}` (U+{:04X}) to be katakana",
				chr,
				chr as u32,
			)
		}

		for chr in KATAKANA_HALF.chars() {
			assert!(
				is_katakana(chr),
				"expected `{}` (U+{:04X}) to be katakana",
				chr,
				chr as u32,
			)
		}

		for chr in KANJI.chars() {
			assert!(
				is_kanji(chr),
				"expected `{}` (U+{:04X}) to be kanji",
				chr,
				chr as u32,
			)
		}

		let all_kana = KATAKANA.to_string() + KATAKANA_HALF + HIRAGANA + BAR_LINE;
		for chr in all_kana.chars() {
			assert!(
				is_kana(chr),
				"expected `{}` (U+{:04X}) to be kana",
				chr,
				chr as u32,
			)
		}

		let all_letters = all_kana + KANJI;
		for chr in all_letters.chars() {
			assert!(
				is_letter(chr),
				"expected `{}` (U+{:04X}) to be letter",
				chr,
				chr as u32,
			)
		}

		for chr in JAPANESE_PUNCTUATION.chars() {
			assert!(
				is_japanese_punctuation(chr),
				"expected `{}` (U+{:04X}) to be a japanese punctuation",
				chr,
				chr as u32,
			)
		}

		let all_marks = JAPANESE_MARK.to_string() + BAR_LINE;
		for chr in all_marks.chars() {
			assert!(
				is_japanese_mark(chr),
				"expected `{}` (U+{:04X}) to be a japanese mark",
				chr,
				chr as u32,
			)
		}

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
