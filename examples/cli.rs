#![feature(or_patterns)]

extern crate kana;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
	println!("\nType strings to translate between hiragana, katakana and romaji:\n");

	let mut rl = Editor::<()>::new();
	loop {
		let input = rl.readline(">> ");
		match input {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				println!();
				println!("   Input:    {}", line.as_str());
				println!("   Hiragana: {}", kana::to_hiragana(line.as_str()));
				println!("   Katakana: {}", kana::to_katakana(line.as_str()));
				println!("   Romaji:   {}", kana::to_romaji(line.as_str()));
				println!();
			}
			Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
				println!();
				break;
			}
			Err(err) => println!("\n   Error: {}\n", err),
		}
	}
}
