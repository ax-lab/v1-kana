//! Unicode related constants for Japanese characters.

/// Codepoint for the first Hiragana character (Hiragana Letter Small A `ぁ`).
pub const HIRAGANA_START: u32 = 0x3041;

/// Codepoint for the last Hiragana character (Hiragana Letter Small Ke `ゖ`).
pub const HIRAGANA_END: u32 = 0x3096;

/// Codepoint for the first Katakana character (Katakana Letter Small A `ァ`).
pub const KATAKANA_START: u32 = 0x30A1;

/// Codepoint for the last Katakana character (Katakana Letter Vo `ヺ`).
pub const KATAKANA_END: u32 = 0x30FA;

// Codepoint for the first small Katakana character (U+31F0 Katakana Letter Small Ku “ㇰ”).
pub const SMALL_KATAKANA_START: u32 = 0x31F0;

// Codepoint for the last small Katakana character (U+31FF Katakana Letter Small Ro “ㇿ”).
pub const SMALL_KATAKANA_END: u32 = 0x31FF;

// Codepoint for the first halfwidth Katakana character (U+FF66 Halfwidth Katakana Letter Wo “ｦ”).
pub const HALF_KATAKANA_START: u32 = 0xFF66;

// Codepoint for the last halfwidth Katakana character (U+FF9D Halfwidth Katakana Letter N “ﾝ”).
pub const HALF_KATAKANA_END: u32 = 0xFF9D;

/// Last Katakana that can be converted directly to Hiragana by offseting
/// (Katakana Letter Small Ke `ヶ`).
pub const KATAKANA_TO_HIRAGANA_END: u32 = 0x30F6;

/// Offset that must be subtracted from a Katakana character to get the
/// respective Hiragana.
///
/// Only valid between `KATAKANA_START` and `KATAKANA_TO_HIRAGANA_END`.
pub const KATAKANA_TO_HIRAGANA_OFFSET_SUB: u32 = KATAKANA_START - HIRAGANA_START;

// Kanji block
// ===========
//
// The basic `CJK Unified Ideographs` block:

/// Codepoint for the first Kanji character (Ideograph one; a, an; alone CJK `一`)
pub const KANJI_START: u32 = 0x4E00;

/// Codepoint for the last Kanji character (Ideograph `龯`)
pub const KANJI_END: u32 = 0x9FAF;

// Kanji extensions
// ----------------
//
// Note that those are rare and uncommon kanji. Extension blocks go all the way
// up to F (with a G planned), but we support only through D because after that
// most characters don't even render.
//
// Source: https://en.wikipedia.org/wiki/CJK_Unified_Ideograph

/// Codepoint for the first Kanji from CJK Unified Ideographs Extension A
/// (`㐀`).
pub const KANJI_START_A: u32 = 0x3400;

/// Codepoint for the last Kanji from CJK Unified Ideographs Extension A
/// (`䶵`).
pub const KANJI_END_A: u32 = 0x4DB5;

/// Codepoint for the first Kanji from CJK Unified Ideographs Extension B
/// (`𠀀`).
pub const KANJI_START_B: u32 = 0x20000;

/// Codepoint for the last Kanji from CJK Unified Ideographs Extension B
/// (`𪛖`).
pub const KANJI_END_B: u32 = 0x2A6D6;

/// Codepoint for the first Kanji from CJK Unified Ideographs Extension C
/// (`𪜀`).
pub const KANJI_START_C: u32 = 0x2A700;

/// Codepoint for the last Kanji from CJK Unified Ideographs Extension C
/// (`𫜴`).
pub const KANJI_END_C: u32 = 0x2B734;

/// Codepoint for the first Kanji from CJK Unified Ideographs Extension D
/// (`𫝀`).
pub const KANJI_START_D: u32 = 0x2B740;

/// Codepoint for the last Kanji from CJK Unified Ideographs Extension D
/// (`𫠝`).
pub const KANJI_END_D: u32 = 0x2B81D;
