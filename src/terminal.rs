#![warn(missing_docs)]

//! Shell like UI over the terminal.
use core::Ingame;
use core::Action;
use std::io;
use std::collections::HashMap;
use std::io::Write;
use std::error::Error;
use std::fmt::Display;
use core::berr;
use core::GameResult;

/// Main Terminal UI type.
pub struct Terminal {
	/// The main game engine
	pub ingame: Ingame,
	/// The registered commands
	pub commands: HashMap<String, Command>,
	/// The prompt string
	pub prompt: String
}

/// A command which is executed 
pub struct Command {
	/// User keyword
	pub keyword: String,
	/// Action executed when the usert types the keyword.
	pub action_fn: Box<Fn(&mut Ingame, &[&str]) -> Result<Action, Box<Error>>>
}




impl Terminal {
	/// Create a new terminal with the given Ingame.
	pub fn new(ingame: Ingame) -> Self {
		Terminal {
			ingame: ingame,
			commands: HashMap::new(),
			prompt: "> ".to_string()
		}
	}

	/// Perform one stop by executing a command
	///
	/// # Error
	/// Returns an error if the input produces an error.
	pub fn step(&mut self, input: &str) -> GameResult<String> {
		// Divide the keywords into the their tokens.
		let keywords: Vec<&str> = input.split(" ").collect();

		// Abort on no input
		if keywords.len() == 0 {
			return Err(berr("Keywords are empty"))
		}

		// Get the command entry according to the first token.
		//
		if let Some(command) = self.commands.get(keywords[0].trim()) {
			// Extruct and run action.
			// If the command replies an Ok, it will contain an action which
			// will be added as one time action.
			// If the command replies an Err, no ingame.step will be called
			// And the error string will be printed.
			let command_fn = &command.action_fn;
			match command_fn(&mut self.ingame, &keywords) {
				Ok(action) =>  {
					self.ingame.add_one_time_action(action);
					self.ingame.step();
					let ingame_error = self.ingame.get_response("err");
					if ingame_error.is_empty() {
						Ok(format!("{}\n", self.ingame.get_response("out")))
					} else {
						Err(berr(format!("Error: {}\n", ingame_error)))
					}
				},
				Err(err) => Err(berr(format!("{}", err)))
			}
		} else {
			// Tell the user, the command was not found.
			Err(berr(
				format!("Could not find command '{}'\n", keywords[0].trim())))
		}
	}

	/// Runs the repl.
	pub fn run(&mut self) {
		print!("Commands: {}\n", self.commands.len());
		loop {
			if self.ingame.get_response("done") != "" {
				break;
			}
			let input = line_input(&self.prompt).expect("IO Error");
			match self.step(&input) {
				Ok(msg) => println!("{}", msg),
				Err(err) => println!("{}", err)
			}
		}
	}

	/// Add a new command to the terminal.
	pub fn add_command(&mut self, command: Command) {
		self.commands.insert(command.keyword.clone(), command);
	}
}

/// Display the prompt and reads a single line from stdin.
///
/// Does not contain a trailing /n.  The prompt does not need a newline,
/// stdout will be flushed.
pub fn line_input<S: Display>(prompt: S) -> Result<String, io::Error> {
	let mut res = String::new();
	print!("{}", prompt);
	io::stdout().flush()?;
	io::stdin().read_line(&mut res)?;
	Ok(res.trim().to_string())
}

/// Requests a multiline String from the user.
///
/// This is for example used to get a description.  The inpot stops when
/// the user adds inputs the term.
pub fn multiline_input<S: Into<String>>(term: S) -> Result<String, io::Error> {
	let term = term.into();
	let mut res = String::new();
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	while input.trim() != term.trim() {
		print!("{:?}, {:?}\n", input, term);
		res.push_str(&input);
		input.clear();
		io::stdin().read_line(&mut input)?;
	}
	Ok(res.trim().to_string())
}