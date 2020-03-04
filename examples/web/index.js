import * as kana from "kana";

kana.greet();
console.log(kana.to_hiragana("hiragana hero"));
console.log(kana.to_katakana("hiragana hero"));

window.kana = kana;
