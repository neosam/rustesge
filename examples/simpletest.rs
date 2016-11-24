extern crate rustesge;
use rustesge::core::Ingame;
use rustesge::terminal::{Terminal};
use rustesge::terminal::Command;

pub fn main() {
	let ingame = Ingame::new("storage".to_string());
	let mut terminal = Terminal::new(ingame);
	let command = Command {
		keyword: "quit".to_string(),
		action_fn: Box::new(| _, _ | {
			Ok(Box::new(| mut ingame, _ | {
				ingame.append_response("done", "true");
				Ok(())
			}))
		})
	};
	let command2 = Command {
		keyword: "echo".to_string(),
		action_fn: Box::new(| _, keywords | {
			let keywords = keywords.join(" ").clone();
			Ok(Box::new(move | mut ingame, _ | { 
				ingame.append_response("out", &keywords);
				Ok(())
			}))
		})
	};
	terminal.add_command(command);
	terminal.add_command(command2);
	print!("Running terminal\n");
	terminal.run();
}