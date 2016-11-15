extern crate rustesge;
use rustesge::core::Ingame;
use rustesge::terminal::Terminal;
use rustesge::terminal::Command;

pub fn main() {
	let ingame = Ingame::new();
	let mut terminal = Terminal::new(ingame);
	let command = Command {
		keyword: "quit".to_string(),
		action_fn: Box::new(| _, _ | {
			Some(Box::new(| mut ingame, _ | {
				ingame.append_response("done", "true");
			}))
		})
	};
	let command2 = Command {
		keyword: "echo".to_string(),
		action_fn: Box::new(| _, keywords | {
			let keywords = keywords.join(" ").clone();
			Some(Box::new(move | mut ingame, _ | { 
				ingame.append_response("out", &keywords);
			}))
		})
	};
	terminal.add_command(command);
	terminal.add_command(command2);
	print!("Running terminal\n");
	terminal.run();
}