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
