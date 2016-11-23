use core::Ingame;
use core::Action;
use std::io;
use std::collections::HashMap;
use std::io::Write;

pub struct Terminal {
	pub ingame: Ingame,
	pub commands: HashMap<String, Command>,
	pub prompt: String
}

pub struct Command {
	pub keyword: String,
	pub action_fn: Box<Fn(&Ingame, &[&str]) -> Option<Action>>
}


impl Terminal {
	pub fn new(ingame: Ingame) -> Self {
		Terminal {
			ingame: ingame,
			commands: HashMap::new(),
			prompt: "> ".to_string()
		}
	}

	pub fn run(&mut self) {
		print!("Commands: {}\n", self.commands.len());
		loop {
			if self.ingame.get_response("done") != "" {
				break;
			}
			print!("{}", self.prompt);
			io::stdout().flush()
				.expect("IO Error");
			let mut input = String::new();
			io::stdin().read_line(&mut input)
				.expect("IO Error");
			let keywords: Vec<&str> = input.split(" ").collect();
			if keywords.len() == 0 {
				break;
			}
			let command_option = self.commands.get(keywords[0].trim());
			if let Some(command) = command_option {
				let command_fn = &command.action_fn;
				let action_option = command_fn(&mut self.ingame, &keywords);
				if let Some(action) = action_option {
					self.ingame.add_one_time_action(action);
					self.ingame.step();
					print!("{}\n", self.ingame.get_response("out"));
					let ingame_error = self.ingame.get_response("err");
					if !ingame_error.is_empty() {
						print!("Error: {}\n", ingame_error);
					}
				}
			} else {
				print!("Could not find command '{}'\n", keywords[0].trim());
			}
		}
	}

	pub fn add_command(&mut self, command: Command) {
		self.commands.insert(command.keyword.clone(), command);
	}
}

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