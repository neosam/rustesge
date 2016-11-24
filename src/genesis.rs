#![warn(missing_docs)]

//! Create a world inside another world
use core::{Storage, Action, Ingame, GameError};
use room::Room;
use actor::Actor;
use base::{BaseGame};
use terminal::{Command, multiline_input};
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

  	Box::new(move |mut ingame, _| {
  		ingame.insert_item(Box::new(room.clone()));
  		Ok(())
  	})
}

/// Create an action which creates a new exit.
///
/// It also creates a new room if the room doesn't exist.
pub fn gen_exit_action<S: Into<String>>(exit_name: S, room_id: S) -> Action {
	let exit_name: String = exit_name.into();
	let room_id: String = room_id.into();
	Box::new(move |mut ingame, _| {
		let mut player_room = ingame.ingame.room_of_player()?;
		if ingame.get_item::<Room>(&room_id).is_none() {
			ingame.insert_item(Box::new(Room::new(room_id.clone())));
		}
		player_room.exits.insert(exit_name.clone(), room_id.clone());
		ingame.insert_item(player_room);
		Ok(())
	})
}

/// Command which generates an exit.
pub fn gen_exit_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, keywords | {
			if keywords.len() < 3 {
				Err(GameError::new("Expected two arguments".to_string()))?;
			}
			Ok(gen_exit_action(keywords[1].trim(), keywords[2].trim()))
		})
	}
}

/// Action to rename a room.
pub fn gen_rename_room_action<S: Into<String>>(name: S) -> Action {
	let name: String = name.into();
	Box::new(move | mut ingame, _ | {
		let mut player_room = ingame.ingame.room_of_player()?;
		player_room.name = name.clone();
		ingame.insert_item(player_room);
		Ok(())
	})
}

/// Action to change the description of a room.
pub fn gen_redescribe_room_action<S: Into<String>>(name: S) -> Action {
	let name: String = name.into();
	Box::new(move | mut ingame, _ | {
		let mut player_room = ingame.ingame.room_of_player()?;
		player_room.description = name.clone();
		ingame.insert_item(player_room);
		Ok(())
	})
}

/// Command to change the description of a room.
pub fn gen_rename_room_cmd<S: Into<String>>(keyword: S) -> Command{
	Command {
		keyword: keyword.into(),
		action_fn: Box::new(|_, keywords | {
			if keywords.len() < 2 {
				Err(GameError::new("Expected one argument".to_string()))?;
			}
			Ok(gen_rename_room_action(keywords[1].trim()))
		})
	}
}

/// Command to change the description of the current room.
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

/// Save the storage to a file at the given path.
pub fn save_world(ingame: &Ingame, path: String) -> Result<(), Box<Error>> {
	let export_str: String = ingame.serialize()?;
	let mut out_file = fs::File::create(&path)?;
	write!(out_file, "{}", export_str)?;

	Ok(())
}

/// Load the storage from the given path.
pub fn load_world(ingame: &mut Ingame, path: String) -> Result<(), Box<Error>> {
	let mut in_file = fs::File::open(&path)?;
	let mut import_str = String::new();
	in_file.read_to_string(&mut import_str)?;
	ingame.from_json(&import_str)?;
	Ok(())
}

/// Command to save the storage.
pub fn save_world_cmd(keyword: String) -> Command {
	Command {
		keyword: keyword,
		action_fn: Box::new(|ingame, _ | {
			let mut name = String::new();
			io::stdin().read_line(&mut name)?;
			save_world(ingame, name.trim().to_string())?;
			Err(GameError::new("".to_string()))?
		})
	}
}

/// Command to load the storage.
pub fn load_world_cmd(keyword: String) -> Command {
	Command {
		keyword: keyword,
		action_fn: Box::new(|mut ingame, _ | {
			let mut name = String::new();
			io::stdin().read_line(&mut name)?;
			load_world(ingame, name.trim().to_string())?;
			Err(GameError::new("".to_string()))?
		})
	}
}

/// Create a very simple worly
pub fn empty_world(player_name: &str, world_name: &str) -> Box<Storage> {
	let player = Actor {
		id: "player-actor".to_string(),
		name: player_name.into(),
		description: "You".to_string()
	};
	let base_game = BaseGame {
		player: "player-actor".to_string()
	};
	let mut room = Room::new("init-room")
				.with_name("Init")
				.with_description("You are in an empty worly");
	room.actors.push("player-actor".to_string());
	Box::new(Storage::new(world_name.to_string())
		.with_item(player)
		.with_item(base_game)
		.with_item(room))
}

/// Create a new empty world and puts it into the player's room.
pub fn gen_empty_world_cmd(keyword: String) -> Command {
		Command {
		keyword: keyword,
		action_fn: Box::new(move |_, _ | {
			let mut player_name = String::new();
			let mut world_name = String::new();
			println!("Player name:");
			io::stdin().read_line(&mut player_name)?;
			println!("World name:");
			io::stdin().read_line(&mut world_name)?;
			Ok(Box::new(move |mut ingame, _ | {
				let world = empty_world(&player_name, &world_name);
				ingame.insert_item_in_player_room(world)
			}))
		})
	}
}