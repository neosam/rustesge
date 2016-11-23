//! Create a world inside another world


use core::{Storage, Action};
use room::Room;
use actor::Actor;
use base::{BaseGame, room_of_player};
use terminal::{Command, multiline_input};

/// Get ste minimal storage required
pub fn initial_genesis(player_name: &str) -> Storage {
	let player = Actor {
		id: "player-actor".to_string(),
		name: player_name.into(),
		description: "You".to_string()
	};
	let base_game = BaseGame {
		player: "player-actor".to_string()
	};
	let mut room = Room::new("genesis-room")
				.with_name("Genesis");
	room.actors.push("player-actor".to_string());
	Storage::new("genesis-stor")
		.with_item(player)
		.with_item(base_game)
		.with_item(room)
}

/// Generates a one time action which creates a new room.
pub fn gen_add_room_action<S, O, O2>(id: S, name: Option<String>, 
				description: Option<String>) -> Action 
		where S: Into<String> {
  	let id: String = id.into();
  	let mut room = Room::new(id);
  	if let Some(name) = name {
  		room.name = name.into();
  	}
  	if let Some(description) = description {
  		room.description = description.into();
  	}

  	Box::new(move |mut ingame, _| ingame.insert_item(Box::new(room.clone())))
}

pub fn gen_exit_action<S: Into<String>>(exit_name: S, room_id: S) -> Action {
	let exit_name: String = exit_name.into();
	let room_id: String = room_id.into();
	Box::new(move |mut ingame, _| {
		let mut player_room = match room_of_player(ingame.ingame) {
			Ok(room) => room,
			Err(msg) => { 
				ingame.append_response("err", &msg); 
				return
			}
		};
		if ingame.get_item::<Room>(&room_id).is_none() {
			ingame.insert_item(Box::new(Room::new(room_id.clone())));
		}
		player_room.exits.insert(exit_name.clone(), room_id.clone());
		ingame.insert_item(player_room);
	})
}

pub fn gen_exit_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, keywords | {
			if keywords.len() < 3 {
				return None
			}
			Some(gen_exit_action(keywords[1].trim(), keywords[2].trim()))
		})
	}
}

pub fn gen_rename_room_action<S: Into<String>>(name: S) -> Action {
	let name: String = name.into();
	Box::new(move | mut ingame, _ | {
		let mut player_room = match room_of_player(ingame.ingame) {
			Ok(room) => room,
			Err(msg) => { 
				ingame.append_response("err", &msg); 
				return
			}
		};
		player_room.name = name.clone();
		ingame.insert_item(player_room);
	})
}
pub fn gen_redescribe_room_action<S: Into<String>>(name: S) -> Action {
	let name: String = name.into();
	Box::new(move | mut ingame, _ | {
		let mut player_room = match room_of_player(ingame.ingame) {
			Ok(room) => room,
			Err(msg) => { 
				ingame.append_response("err", &msg); 
				return
			}
		};
		player_room.description = name.clone();
		ingame.insert_item(player_room);
	})
}
pub fn gen_rename_room_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, keywords | {
			if keywords.len() < 2 {
				return None
			}
			Some(gen_rename_room_action(keywords[1].trim()))
		})
	}
}

pub fn gen_redescribe_room_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, _ | {
			print!("Write a multiline text, terminate with END\n");
			let description = multiline_input("END");
			match description {
				Ok(description) => Some(gen_redescribe_room_action(description)),
				Err(err) => { print!("{:?}\n", err); None }
			}
		})
	}
}

