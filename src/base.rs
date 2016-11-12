use core::{Ingame, MutIngame};
use core::Item;
use actor::Actor;
use room::Room;

use std::result::Result;

pub fn room_of_actor(ingame: &Ingame, actor: &Actor) -> Option<Box<Room>> {
	let id = &actor.id;
	let rooms: Vec<Box<Room>> = ingame.all_of_type();
	for room in rooms {
		if room.actors.contains(id) {
			return Some(room)
		}
	}
	None
}

pub fn remove_actor_from_room(ingame: &mut MutIngame, actor: &Actor, mut room: Box<Room>) {
	let id = &actor.id;
	room.actors = room.actors.iter()
		.map(|x| x.to_string())
		.filter(|x| x != id)
		.collect();
	ingame.insert_item(room);
}

pub fn warp_actor(ingame: &mut MutIngame, actor: &Actor, mut room: Box<Room>) {
	let src_room: Option<Box<Room>> = room_of_actor(ingame.ingame, actor);
	if src_room.is_some() {
		remove_actor_from_room(ingame, actor, src_room.unwrap());
	}
	room.actors.push(actor.id.clone());
	ingame.insert_item(room);
}

pub fn move_actor(ingame: &mut MutIngame, actor: &Actor, exit_name: &str) -> Result<(), String> {
	let actor_room = room_of_actor(ingame.ingame, actor);
	if actor_room.is_none() {
		return Err("Actor is not in any room".to_string());
	}
	let actor_room = actor_room.unwrap();
	if !actor_room.exits.contains_key(exit_name) {
		return Err("Exit not in actors room".to_string());
	}
	let dest_room: Option<Box<Room>> = ingame.get_item(actor_room.exits.get(&actor.id).unwrap());
	if dest_room.is_none() {
		return Err("Dest room key not found in storage".to_string());
	}
	let dest_room = dest_room.unwrap();
	warp_actor(ingame, actor, dest_room);
	Ok(())
}

pub fn exits_in_room(ingame: &Ingame, room: &Room) -> Vec<(String, Box<Room>)> {
	room.exits.iter()
		.map(|(label, room_id)| (label.to_string(), ingame.get_item(room_id)))
		.filter(| &(_, ref room_option) | room_option.is_some())
		.map(| (label, room_option) | (label, room_option.unwrap()))
		.collect()
}

pub fn items_in_room(ingame: &Ingame, room: &Room) -> Vec<Box<Item>> {
	room.items.iter()
		.map(|x| ingame.get_item(x))
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect()
}

pub fn actors_in_room(ingame: &Ingame, room: &Room) -> Vec<Box<Actor>> {
	room.actors.iter()
		.map(|x| ingame.get_item(x))
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect()
}