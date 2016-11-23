//! Create a world inside another world


use core::{Storage, Action, Ingame};
use room::Room;
use actor::Actor;
use base::{BaseGame, room_of_player};
use terminal::{Command, multiline_input, MsgError};
use std::io;
use std::io::{Write, Read};
use std::error::Error;
use std::fs;


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
				Err(MsgError::new("Expected two arguments".to_string()))?;
			}
			Ok(gen_exit_action(keywords[1].trim(), keywords[2].trim()))
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
				Err(MsgError::new("Expected one argument".to_string()))?;
			}
			Ok(gen_rename_room_action(keywords[1].trim()))
		})
	}
}

pub fn gen_redescribe_room_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, _ | {
			print!("Write a multiline text, terminate with END\n");
			let description = multiline_input("END")?;
			Ok(gen_redescribe_room_action(description))
		})
	}
}

pub fn save_world(ingame: &Ingame, path: String) -> Result<(), Box<Error>> {
	let export_str: String = ingame.serialize()?;
	let mut out_file = fs::File::create(&path)?;
	write!(out_file, "{}", export_str)?;

	Ok(())
}
pub fn load_world(ingame: &mut Ingame, path: String) -> Result<(), Box<Error>> {
	let mut in_file = fs::File::open(&path)?;
	let mut import_str = String::new();
	in_file.read_to_string(&mut import_str)?;
	ingame.from_json(&import_str)?;
	Ok(())
}

pub fn save_world_cmd(keyword: String) -> Command {
	Command {
		keyword: keyword,
		action_fn: Box::new(|ingame, _ | {
			let mut name = String::new();
			io::stdin().read_line(&mut name)?;
			save_world(ingame, name.trim().to_string())?;
			Err(MsgError::new("".to_string()))?
		})
	}
}
pub fn load_world_cmd(keyword: String) -> Command {
	Command {
		keyword: keyword,
		action_fn: Box::new(|mut ingame, _ | {
			let mut name = String::new();
			io::stdin().read_line(&mut name)?;
			load_world(ingame, name.trim().to_string())?;
			Err(MsgError::new("".to_string()))?
		})
	}
}
