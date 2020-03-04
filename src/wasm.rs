use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
	alert("Hello from wasm!");
}

#[wasm_bindgen]
pub fn set_panic_hook() {
	// When the `console_error_panic_hook` feature is enabled, we can call the
	// `set_panic_hook` function at least once during initialization, and then
	// we will get better error messages if our code ever panics.
	//
	// For more details see
	// https://github.com/rustwasm/console_error_panic_hook#readme
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn to_katakana(input: String) -> String {
	super::to_katakana(input)
}

#[wasm_bindgen]
pub fn to_hiragana(input: String) -> String {
	super::to_hiragana(input)
}

#[wasm_bindgen]
pub fn to_romaji(input: String) -> String {
	super::to_romaji(input)
}

#[wasm_bindgen]
pub fn is_hiragana(input: String) -> bool {
	input.chars().all(|c| super::is_hiragana(c))
}

#[wasm_bindgen]
pub fn is_katakana(input: String) -> bool {
	input.chars().all(|c| super::is_katakana(c))
}

#[wasm_bindgen]
pub fn is_kanji(input: String) -> bool {
	input.chars().all(|c| super::is_kanji(c))
}

#[wasm_bindgen]
pub fn is_kana(input: String) -> bool {
	input.chars().all(|c| super::is_kana(c))
}

#[wasm_bindgen]
pub fn is_letter(input: String) -> bool {
	input.chars().all(|c| super::is_letter(c))
}

#[wasm_bindgen]
pub fn is_japanese_mark(input: String) -> bool {
	input.chars().all(|c| super::is_japanese_mark(c))
}

#[wasm_bindgen]
pub fn is_japanese_punctuation(input: String) -> bool {
	input.chars().all(|c| super::is_japanese_punctuation(c))
}
