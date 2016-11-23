extern crate rustesge;
use rustesge::core::Ingame;
use rustesge::core::Storage;
use rustesge::terminal::Terminal;
use rustesge::terminal::Command;
use rustesge::room::Room;
use rustesge::actor::Actor;
use rustesge::base::BaseGame;
use rustesge::base;
use std::collections::HashMap;
use rustesge::genesis;

pub fn main() {
	let mut room1 = Room {
		id: "room1".to_string(),
		name: "Room1".to_string(),
		description: "This is room 1".to_string(),
		items: Vec::new(),
		actors: Vec::new(),
		exits: HashMap::new()
	};
	let mut room2 = Room {
		id: "room2".to_string(),
		name: "Room2".to_string(),
		description: "This is room 2".to_string(),
		items: Vec::new(),
		actors: Vec::new(),
		exits: HashMap::new()
	};
	room1.exits.insert("out".to_string(), "room2".to_string());
	room2.exits.insert("in".to_string(), "room1".to_string());
	let lalala = Actor {
		id: "lalala".to_string(),
		name: "Lalala".to_string(),
		description: "Lalala the choco elfin.".to_string()
	};
	room1.actors.push("lalala".to_string());
	let state = BaseGame {
		player: "lalala".to_string()
	};
	let mut storage = Storage::new("storage");
	storage.insert(Box::new(room1));
	storage.insert(Box::new(room2));
	storage.insert(Box::new(lalala));
	storage.insert(Box::new(state));
	storage = genesis::initial_genesis("God");
	let base_package = base::gen_esge_package();
	let packages = vec![base_package];


	let quit_cmd = Command {
		keyword: "quit".to_string(),
		action_fn: Box::new(| _, _ | {
			Some(Box::new(| mut ingame, _ | {
				ingame.append_response("done", "true");
			}))
		})
	};	
	let look_cmd = Command {
		keyword: "look".to_string(),
		action_fn: Box::new(| _, _ | {
			Some(base::gen_display_current_room_action())
		})
	};
	let store_cmd = Command {
		keyword: "store".to_string(),
		action_fn: Box::new(| _, _ | {
			Some(Box::new(| mut ingame, _ | {
				match ingame.ingame.serialize() {
					Ok(res) => ingame.append_response("out", &res),
					Err(err) => ingame.append_response("err", &format!("{:?}", err))
				}
			}))
		})
	};
	let go_cmd = Command {
		keyword: "go".to_string(),
		action_fn: Box::new(| _, keywords | {
			if keywords.len() <= 1 {
				print!("Which direction?\n");
				None
			} else {
				Some(base::gen_move_player_action(keywords[1].trim().to_string()))
			}
		})
	};
	let error_cmd = Command {
		keyword: "err".to_string(),
		action_fn: Box::new(| _, _ | {
			Some(Box::new(| mut ingame, _ | {
				ingame.append_response("err", "Test\n");
			}))
		})
	};

	match base::init_packages(storage, packages) {
		Ok(ingame) => {
			let mut terminal = Terminal::new(ingame);
			terminal.add_command(quit_cmd);
			terminal.add_command(look_cmd);
			terminal.add_command(go_cmd);
			terminal.add_command(error_cmd);
			terminal.add_command(store_cmd);
			terminal.add_command(genesis::gen_exit_cmd("add_exit"));
			terminal.add_command(genesis::gen_rename_room_cmd("rename_room"));
			terminal.add_command(genesis::gen_redescribe_room_cmd("redescribe_room"));
			print!("Running terminal\n");
			terminal.run();
		},
		Err(msg) => print!("Could not create ingame: {}", msg)
	}
}