//! Japanese character (kana and kanji) utilities.
//!
//! This library provides fast translation between Hiragana, Katakana and Romaji
//! as well as utility functions to test different Japanese characters.

extern crate fnv;

#[macro_use]
extern crate lazy_static;

// CharCode References
// http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
// http://unicode-table.com

mod constants;
mod table;
mod util;

mod is;
pub use is::*;

mod to;
pub use to::*;

mod kind;
pub use kind::*;
