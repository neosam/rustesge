#![warn(missing_docs)]

//! Genesis frontend

use core::GameResult;
use terminal::{Terminal, line_input};
use std::path::{PathBuf};

/// Terminal frontend implementation
pub struct TerminalGenesis {
	/// Contains the terminal implementation
	pub terminal: Terminal,

	/// Where everything should be saved
	pub base_path: PathBuf
}

impl TerminalGenesis {
	/// Creates a new instance
	pub fn new(t: Terminal, path: PathBuf) -> Self {
		TerminalGenesis {
			terminal: t,
			base_path: path
		}
	}

	// Creates a new instance by asking the user to login.
	/*pub fn with_login(base_path: String) -> Self {
		let username = input_word("Username: ", "Please try again!\n");
		let 
	}*/
}

/// Checks if a boolean is a valid word (digit or letter)
pub fn is_valid_word(c: char) -> bool {
	(c >= '0' && c <= '9') ||
		(c >= 'a' && c <= 'z') ||
		(c >= 'A' && c <= 'Z') ||
		c == '-' || c == '_'
}

/// Asks for an input of just a single word.
pub fn input_word(prompt: &str, error_msg: &str) -> GameResult<String> {
	let mut done = false;
	let mut msg = String::new();
	while !done {
		msg = line_input(prompt)?;
		if !msg.is_empty() && msg.as_str().chars().all(|x| is_valid_word(x)) {
			done = true;
		} else {
			print!("{}", error_msg);
		}
	}
	Ok(msg)
}