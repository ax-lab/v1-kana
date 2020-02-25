//! Japanese character manipulation and conversion module
//!
//! This is largely based on https://github.com/PSeitz/wana_kana_rust but
//! provides an API specifically design for this application.

use super::constants::*;
use super::table::*;
use super::util::*;

/// Converts the input string into hiragana. Unknown characters just pass
/// through unchanged.
///
/// Supports mapping romaji and katakana.
pub fn to_hiragana<S: AsRef<str>>(input: S) -> String {
	let input = input.as_ref();
	let mut src = input;
	let mut out = String::with_capacity(src.len());

	while src.len() > 0 {
		let mut chars = src.char_indices();
		let (_, next) = chars.next().unwrap(); // next character
		let (size, _) = chars.next().unwrap_or((src.len(), ' ')); // size of next

		let mut skip = size;
		let mut done = false;

		if char_in_range(next, KATAKANA_START, KATAKANA_TO_HIRAGANA_END) {
			// For katakana we can convert directly just by offseting the code
			let code = (next as u32) - KATAKANA_TO_HIRAGANA_OFFSET_SUB;
			let hiragana = unsafe { std::char::from_u32_unchecked(code) };
			out.push(hiragana);
			done = true;
		} else if !char_in_range(next, HIRAGANA_START, HIRAGANA_END) {
			// Handle the double consonant case
			let b = src.as_bytes();
			if b.len() >= 2 {
				let c = b[0] as char;
				if c != 'n' && c != 'N' && is_consonant(c, true) && b[0] == b[1] {
					out.push('っ');
					done = true;
				}
			}

			if !done {
				// Try to convert all chunk sizes, starting from largest down to 1.
				let max_chunk = if next == ':'
					|| (next >= 'a' && next <= 'z')
					|| (next >= 'A' && next <= 'Z')
				{
					// Multi-char lookup keys either start with A-Z or `:`
					*TO_HIRAGANA_MAX_CHUNK
				} else {
					1
				};
				for len in (1..=max_chunk).rev() {
					let chunk = get_prefix(src, len);
					if let Some(kana) = TO_HIRAGANA.get(chunk) {
						out.push_str(kana);
						skip = chunk.len();
						done = true;
						break;
					}
				}
			}
		}

		// If could not find a conversion, just pass through the character.
		if !done {
			out.push(next);
		}

		src = &src[skip..];
	}

	out
}

/// Converts the input string into katakana. Unknown characters just pass
/// through unchanged.
///
/// Supports mapping romaji and hiragana.
pub fn to_katakana<S: AsRef<str>>(input: S) -> String {
	let hiragana = to_hiragana(input);
	let mut out = String::with_capacity(hiragana.len());
	for chr in hiragana.chars() {
		out.push(hiragana_to_katakana(chr));
	}

	out
}

/// Converts any kana in the input to romaji.
///
/// Note that this will pass through interpunct (`・`) marks. Other Japanese
/// punctuation are converted to ASCII variants.
pub fn to_romaji<S: AsRef<str>>(input: S) -> String {
	// Representation for `っ` when it is not a valid double consonant.
	const SMALL_TSU_REPR: char = '\'';

	let mut was_small_tsu = false;

	let mut src = input.as_ref();
	let mut out = String::with_capacity(src.len());
	while src.len() > 0 {
		let mut chars = src.char_indices();
		let (_, next) = chars.next().unwrap(); // next character
		let (size, _) = chars.next().unwrap_or((src.len(), ' ')); // size of next

		let mut skip = size;
		let mut done = false;

		if next == 'っ' || next == 'ッ' {
			if was_small_tsu {
				out.push(SMALL_TSU_REPR); // Case of repeated `っ`
			}
			was_small_tsu = true;
			done = true;
		} else if TO_ROMAJI_CHARS.contains(&next) {
			// Try to convert all chunk sizes down to 1
			for len in (1..=*TO_ROMAJI_MAX_CHUNK).rev() {
				let chunk = get_prefix(src, len);
				if let Some(romaji) = TO_ROMAJI.get(chunk) {
					if was_small_tsu {
						if let Some(doubled) = romaji.chars().next() {
							if is_consonant(doubled, true) {
								was_small_tsu = false;
								out.push(doubled);
							}
						}
						if was_small_tsu {
							out.push(SMALL_TSU_REPR);
							was_small_tsu = false;
						}
					}
					out.push_str(romaji);
					skip = chunk.len();
					done = true;
					break;
				}
			}
		}

		// If could not find a conversion, just pass through the character.
		if !done {
			if was_small_tsu {
				out.push(SMALL_TSU_REPR);
				was_small_tsu = false;
			}
			out.push(next);
		}

		src = &src[skip..];
	}

	if was_small_tsu {
		out.push(SMALL_TSU_REPR);
	}

	out
}

// spell-checker: disable

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_hiragana() {
		fn check(kana: &str, input: &str) {
			assert_eq!(kana, to_hiragana(input), "input `{}`", input);
			assert_eq!(
				kana.to_uppercase(),
				to_hiragana(input.to_uppercase()),
				"input `{}`",
				input
			);
			assert_eq!(
				kana.to_lowercase(),
				to_hiragana(input.to_lowercase()),
				"input `{}`",
				input
			);
		}

		check("", "");
		check("そうしんうぃんどう", "そうしんウィンドウ");

		// Katakana
		const H: &str = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ";
		const K: &str = "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶ";
		check(H, K);
		check(H, H);

		// Romaji
		const D: &str = "しゃぎゃつっじゃあんなん んあんんざ xzm";
		const S: &str = "shyagyatsuxtujaannan n'annza xzm";
		check(D, S);

		// Pass through punctuation
		check("・ー～", "・ー～");
		check("あ：ば", "A: BA"); // `: ` to `：`

		// Weird katakana
		check("ゔぁ ゔぃ ゔ ゔぇ ゔぉ", "ヷ ヸ ヴ ヹ ヺ");

		// Hepburn style romaji and variation
		check("あーいーうーえーおー", "āīūēō");
		check("あーいーうーえーおー", "âîûêô");

		// Double consonants
		check("ばっば", "babba");
		check("かっか", "cacca");
		check("ちゃっちゃ", "chaccha");
		check("だっだ", "dadda");
		check("ふっふ", "fuffu");
		check("がっが", "gagga");
		check("はっは", "hahha");
		check("じゃっじゃ", "jajja");
		check("かっか", "kakka");
		check("らっら", "lalla");
		check("まっま", "mamma");
		check("なんな", "nanna");
		check("ぱっぱ", "pappa");
		check("くぁっくぁ", "qaqqa");
		check("らっら", "rarra");
		check("さっさ", "sassa");
		check("しゃっしゃ", "shassha");
		check("たった", "tatta");
		check("つっつ", "tsuttsu");
		check("ゔぁっゔぁ", "vavva");
		check("わっわ", "wawwa");
		check("やっや", "yayya");
		check("ざっざ", "zazza");

		// Additional kana tests from wana-kana
		check("おなじ", "onaji");
		check("ぶっつうじ", "buttsuuji");
		check("わにかに", "WaniKani");
		check(
			"わにかに あいうえお 鰐蟹 12345 @#$%",
			"ワニカニ AiUeO 鰐蟹 12345 @#$%",
		);
		check("座禅「ざぜん」すたいる", "座禅‘zazen’スタイル");
		check("ばつげーむ", "batsuge-mu");

		let all_kana = vec![
			("ァ", "ぁ", ""),
			("ア", "あ", "a"),
			("ィ", "ぃ", ""),
			("イ", "い", "i"),
			("ゥ", "ぅ", ""),
			("ウ", "う", "u"),
			("ェ", "ぇ", ""),
			("エ", "え", "e"),
			("ォ", "ぉ", ""),
			("オ", "お", "o"),
			("カ", "か", "ka"),
			("ガ", "が", "ga"),
			("キ", "き", "ki"),
			("ギ", "ぎ", "gi"),
			("ク", "く", "ku"),
			("グ", "ぐ", "gu"),
			("ケ", "け", "ke"),
			("ゲ", "げ", "ge"),
			("コ", "こ", "ko"),
			("ゴ", "ご", "go"),
			("サ", "さ", "sa"),
			("ザ", "ざ", "za"),
			("シ", "し", "shi"),
			("ジ", "じ", "ji"),
			("ス", "す", "su"),
			("ズ", "ず", "zu"),
			("セ", "せ", "se"),
			("ゼ", "ぜ", "ze"),
			("ソ", "そ", "so"),
			("ゾ", "ぞ", "zo"),
			("タ", "た", "ta"),
			("ダ", "だ", "da"),
			("チ", "ち", "chi"),
			("ヂ", "ぢ", "di"),
			("ッ", "っ", ""),
			("ツ", "つ", "tsu"),
			("ヅ", "づ", "du"),
			("テ", "て", "te"),
			("デ", "で", "de"),
			("ト", "と", "to"),
			("ド", "ど", "do"),
			("ナ", "な", "na"),
			("ニ", "に", "ni"),
			("ヌ", "ぬ", "nu"),
			("ネ", "ね", "ne"),
			("ノ", "の", "no"),
			("ハ", "は", "ha"),
			("バ", "ば", "ba"),
			("パ", "ぱ", "pa"),
			("ヒ", "ひ", "hi"),
			("ビ", "び", "bi"),
			("ピ", "ぴ", "pi"),
			("フ", "ふ", "fu"),
			("ブ", "ぶ", "bu"),
			("プ", "ぷ", "pu"),
			("ヘ", "へ", "he"),
			("ベ", "べ", "be"),
			("ペ", "ぺ", "pe"),
			("ホ", "ほ", "ho"),
			("ボ", "ぼ", "bo"),
			("ポ", "ぽ", "po"),
			("マ", "ま", "ma"),
			("ミ", "み", "mi"),
			("ム", "む", "mu"),
			("メ", "め", "me"),
			("モ", "も", "mo"),
			("ャ", "ゃ", ""),
			("ヤ", "や", "ya"),
			("ュ", "ゅ", ""),
			("ユ", "ゆ", "yu"),
			("ョ", "ょ", ""),
			("ヨ", "よ", "yo"),
			("ラ", "ら", "ra"),
			("リ", "り", "ri"),
			("ル", "る", "ru"),
			("レ", "れ", "re"),
			("ロ", "ろ", "ro"),
			("ヮ", "ゎ", ""),
			("ワ", "わ", "wa"),
			("ヰ", "ゐ", ""),
			("", "うぃ", "wi"),
			("ヱ", "ゑ", ""),
			("", "うぇ", "we"),
			("ヲ", "を", "wo"),
			("ン", "ん", "n"),
			("ヴ", "ゔ", "vu"),
			("ヵ", "ゕ", ""),
			("ヶ", "ゖ", ""),
			("ヷ", "ゔぁ", "va"),
			("ヸ", "ゔぃ", "vi"),
			("ヹ", "ゔぇ", "ve"),
			("ヺ", "ゔぉ", "vo"),
			("・", "・", "/"),
			("ー", "ー", "-"),
			("ヽ", "ヽ", ""),
			("ヾ", "ヾ", ""),
			("ヿ", "こと", "koto"),
			("゛", "゛", ""),
			("゜", "゜", ""),
			("ゝ", "ゝ", ""),
			("ゞ", "ゞ", ""),
			("ゟ", "より", "yori"),
		];
		for (katakana, hiragana, romaji) in all_kana {
			if romaji.len() > 0 {
				check(hiragana, romaji);
			}
			if katakana.len() > 0 {
				check(hiragana, katakana);
			}
		}
	}

	#[test]
	fn test_to_romaji() {
		fn check(kana: &str, romaji: &str) {
			assert_eq!(romaji, to_romaji(kana), "kana: `{}`", kana);
		}

		check("", "");
		check("そうしんウィンドウ", "soushinwindou");
		check("ああんいぇああ", "aan'yeaa");
		check("ヷヸヴヹヺ ゔぁゔぃゔゔぇゔぉ", "vavivuvevo vavivuvevo");

		//
		// Reversed tests from to_hiragana
		//

		// Hiragana
		const D: &str = "しゃぎゃつっじゃあんなん　んあんんざ　xzm";
		const S: &str = "shagyatsujjaannan n'annza xzm";

		// Long vogals
		check("あーいーうーえーおー", "āīūēō");

		// Double consonants
		check("ばっば", "babba");
		check("かっか", "kakka");
		check("ちゃっちゃ", "chaccha");
		check("だっだ", "dadda");
		check("ふっふ", "fuffu");
		check("がっが", "gagga");
		check("はっは", "hahha");
		check("じゃっじゃ", "jajja");
		check("かっか", "kakka");
		check("まっま", "mamma");
		check("なんな", "nanna");
		check("ぱっぱ", "pappa");
		check("くぁっくぁ", "qwaqqwa");
		check("らっら", "rarra");
		check("さっさ", "sassa");
		check("しゃっしゃ", "shassha");
		check("たった", "tatta");
		check("つっつ", "tsuttsu");
		check("ゔぁっゔぁ", "vavva");
		check("わっわ", "wawwa");
		check("やっや", "yayya");
		check("ざっざ", "zazza");

		// Archaic
		check("ゐゑ ゟ ヿ", "wiwe yori koto");

		// Small tsu at the end of words
		check("ふっ", "fu'");
		check("ふっ ふっ", "fu' fu'");
		check("ぎゃっ！", "gya'!");
		check("っっべあっ…ぎゃっあっあっっっ！っx", "'bbea'…gya'a'a'''!'x");

		// Additional kana tests from wana-kana
		check("おなじ", "onaji");
		check("ぶっつうじ", "buttsuuji");
		check("わにかに", "wanikani");
		check(
			"わにかに あいうえお 鰐蟹 12345 @#$%",
			"wanikani aiueo 鰐蟹 12345 @#$%",
		);
		check("座禅「ざぜん」すたいる", "座禅‘zazen’sutairu");
		check("ばつげーむ", "batsuge-mu");

		check(D, S);

		//
		// Tests from wana-kana
		//

		// Quick Brown Fox Hiragana to Romaji
		check("いろはにほへと", "irohanihoheto");
		check("ちりぬるを", "chirinuruwo");
		check("わかよたれそ", "wakayotareso");
		check("つねならむ", "tsunenaramu");
		check("うゐのおくやま", "uwinookuyama");
		check("けふこえて", "kefukoete");
		check("あさきゆめみし", "asakiyumemishi");
		check("ゑひもせすん", "wehimosesun");

		// Base cases:

		// Convert katakana to romaji"
		check("ワニカニ　ガ　スゴイ　ダ", "wanikani ga sugoi da");
		// Convert hiragana to romaji"
		check("わにかに　が　すごい　だ", "wanikani ga sugoi da");
		// Convert mixed kana to romaji"
		check("ワニカニ　が　すごい　だ", "wanikani ga sugoi da");
		// Doesn't mangle the long dash 'ー' or slashdot '・'"
		check("罰ゲーム・ばつげーむ", "罰ge-mu/batsuge-mu");
		// Spaces must be manually entered"

		// Double ns and double consonants:

		// Double and single n"
		check("きんにくまん", "kinnikuman");
		// N extravaganza"
		check("んんにんにんにゃんやん", "nnninninnyan'yan");
		// Double consonants"
		check(
			"かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ",
			"kappa tatta shusshu chaccha yattsu",
		);

		// Small kana:

		// Small tsu doesn't transliterate"
		check("っ", "'");
		// Small ya"
		check("ゃ", "ya");
		// Small yu"
		check("ゅ", "yu");
		// Small yo"
		check("ょ", "yo");
		// Small a"
		check("ぁ", "a");
		// Small i"
		check("ぃ", "i");
		// Small u"
		check("ぅ", "u");
		// Small e"
		check("ぇ", "e");
		// Small o"
		check("ぉ", "o");
		// Small ke (ka)" - https://en.wikipedia.org/wiki/Small_ke
		check("ヶ", "ka");
		// Small ka"
		check("ヵ", "ka");
		// Small wa"
		check("ゎ", "wa");

		// Apostrophes in vague consonant vowel combos:

		check("おんよみ", "on'yomi");
		check("んよ んあ んゆ", "n'yo n'a n'yu");

		let all_kana = vec![
			("ァ", "ぁ", "a"),
			("ア", "あ", "a"),
			("ィ", "ぃ", "i"),
			("イ", "い", "i"),
			("ゥ", "ぅ", "u"),
			("ウ", "う", "u"),
			("ェ", "ぇ", "e"),
			("エ", "え", "e"),
			("ォ", "ぉ", "o"),
			("オ", "お", "o"),
			("カ", "か", "ka"),
			("ガ", "が", "ga"),
			("キ", "き", "ki"),
			("ギ", "ぎ", "gi"),
			("ク", "く", "ku"),
			("グ", "ぐ", "gu"),
			("ケ", "け", "ke"),
			("ゲ", "げ", "ge"),
			("コ", "こ", "ko"),
			("ゴ", "ご", "go"),
			("サ", "さ", "sa"),
			("ザ", "ざ", "za"),
			("シ", "し", "shi"),
			("ジ", "じ", "ji"),
			("ス", "す", "su"),
			("ズ", "ず", "zu"),
			("セ", "せ", "se"),
			("ゼ", "ぜ", "ze"),
			("ソ", "そ", "so"),
			("ゾ", "ぞ", "zo"),
			("タ", "た", "ta"),
			("ダ", "だ", "da"),
			("チ", "ち", "chi"),
			("ヂ", "ぢ", "di"),
			("ッ", "っ", "'"),
			("ツ", "つ", "tsu"),
			("ヅ", "づ", "du"),
			("テ", "て", "te"),
			("デ", "で", "de"),
			("ト", "と", "to"),
			("ド", "ど", "do"),
			("ナ", "な", "na"),
			("ニ", "に", "ni"),
			("ヌ", "ぬ", "nu"),
			("ネ", "ね", "ne"),
			("ノ", "の", "no"),
			("ハ", "は", "ha"),
			("バ", "ば", "ba"),
			("パ", "ぱ", "pa"),
			("ヒ", "ひ", "hi"),
			("ビ", "び", "bi"),
			("ピ", "ぴ", "pi"),
			("フ", "ふ", "fu"),
			("ブ", "ぶ", "bu"),
			("プ", "ぷ", "pu"),
			("ヘ", "へ", "he"),
			("ベ", "べ", "be"),
			("ペ", "ぺ", "pe"),
			("ホ", "ほ", "ho"),
			("ボ", "ぼ", "bo"),
			("ポ", "ぽ", "po"),
			("マ", "ま", "ma"),
			("ミ", "み", "mi"),
			("ム", "む", "mu"),
			("メ", "め", "me"),
			("モ", "も", "mo"),
			("ャ", "ゃ", "ya"),
			("ヤ", "や", "ya"),
			("ュ", "ゅ", "yu"),
			("ユ", "ゆ", "yu"),
			("ョ", "ょ", "yo"),
			("ヨ", "よ", "yo"),
			("ラ", "ら", "ra"),
			("リ", "り", "ri"),
			("ル", "る", "ru"),
			("レ", "れ", "re"),
			("ロ", "ろ", "ro"),
			("ヮ", "ゎ", "wa"),
			("ワ", "わ", "wa"),
			("ヰ", "ゐ", "wi"),
			("ヱ", "ゑ", "we"),
			("ヲ", "を", "wo"),
			("ン", "ん", "n"),
			("ヴ", "ゔ", "vu"),
			("ヵ", "ゕ", "ka"),
			("ヶ", "ゖ", "ka"), // Note that small ke is pronounced as ka (https://en.wikipedia.org/wiki/Small_ke)
			("ヷ", "", "va"),
			("ヸ", "", "vi"),
			("ヹ", "", "ve"),
			("ヺ", "", "vo"),
			("・", "", "/"),
			("ー", "", "-"),
			("ヽ", "", "ヽ"),
			("ヾ", "", "ヾ"),
			("ヿ", "", "koto"),
			("", "゛", "゛"),
			("", "゜", "゜"),
			("", "ゝ", "ゝ"),
			("", "ゞ", "ゞ"),
			("", "ゟ", "yori"),
		];
		for (katakana, hiragana, val) in all_kana {
			if katakana.len() > 0 {
				check(katakana, val);
			}
			if hiragana.len() > 0 {
				check(hiragana, val);
			}
		}
	}
}
