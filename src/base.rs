use core;
use core::{Ingame, MutIngame, GameResult, GameError, Itemizeable, gerr, berr};
use core::Item;
use actor::Actor;
use room::Room;

use std::result::Result;

pub fn room_of_actor(ingame: &Ingame, actor: &Actor) -> GameResult<Box<Room>> {
	let id = &actor.id;
	let rooms = ingame.all_of_type::<Room>();
	for room in rooms {
		if room.actors.contains(id) {
			return Ok(room)
		}
	}
	Err(berr("Room not found"))
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
	if let Ok(src_room) = room_of_actor(ingame.ingame, actor) {
		remove_actor_from_room(ingame, actor, src_room);
	}
	room.actors.push(actor.id.clone());
	ingame.insert_item(room);
}

pub fn move_actor(ingame: &mut MutIngame, actor: &Actor, exit_name: &str) -> GameResult<()> {
	let actor_room = room_of_actor(ingame.ingame, actor)?;
	let dest_room_name = actor_room.exits.get(exit_name)
			.ok_or(gerr("Could not find exit"))?;
	let dest_room: Box<Room> = ingame.get_item(dest_room_name)
			.ok_or(gerr("Could not get the destination room"))?;
	warp_actor(ingame, actor, dest_room.clone());
	display_room(ingame, dest_room);
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


pub type PlausabilityCheck = Box<Fn(&Ingame) -> Option<String>>;

pub struct EsgePackage {
	init_action: core::Action,
	plausability_check: PlausabilityCheck
}

pub fn init_packages(storage: core::Storage, packages: Vec<EsgePackage>) -> Result<Ingame, String> {
	let mut ingame = Ingame::with_storage(storage);
	for package in packages {
		let plausability_check = &package.plausability_check;
		let result = plausability_check(&ingame);
		if result.is_some() {
			return Err(result.unwrap())
		}
		ingame.add_action(package.init_action);
	}
	Ok(ingame)
}


pub struct BaseGame {
	pub player: String
}

impl core::Itemizeable for BaseGame {
	fn from_item(item: &Item) -> Option<Box<Self>> {
		Some(Box::new(BaseGame {
			player: item.item_meta.get("player").cloned().unwrap_or(String::new())
		}))
	}
	fn to_item(&self) -> core::Item {
		let mut item = core::Item::new("base_game".to_string(), "base_game".to_string());
		self.merge_into_item(&mut item);
		item
	}
	fn merge_into_item(&self, item: &mut Item) {
		item.item_meta.insert("player".to_string(), self.player.clone());
	}
	fn get_id(&self) -> &str {
		"base_game"
	}
}

pub fn get_player(ingame: &Ingame) -> GameResult<Box<Actor>> {
	let base_game = ingame.get_item::<BaseGame>("base_game")
			.ok_or(GameError::new("Could not find base_game"))?;
	ingame.get_item::<Actor>(&base_game.player)
			.ok_or(berr(format!("Player not found: {}", base_game.player)))
}

pub fn room_of_player(ingame: &Ingame) -> GameResult<Box<Room>> {
	let player = get_player(ingame)?;
	room_of_actor(ingame, &*player)
}

pub fn display_room(ingame: &mut MutIngame, room: Box<Room>) {
	ingame.append_response("out", "Room: ");
	ingame.append_response("out", &room.name);
	ingame.append_response("out", "\n");
	ingame.append_response("out", &room.description);
	ingame.append_response("out", "\n");
	if !room.items.is_empty() {
		ingame.append_response("out", "Items: ");
		for item in &room.items {
			ingame.append_response("out", &item);
			ingame.append_response("out", " ");
		}
		ingame.append_response("out", "\n");
	}
	if !room.exits.is_empty() {
		ingame.append_response("out", "Directions: ");
		let keys = room.exits.keys();
		for key in keys {
			ingame.append_response("out", key);
			ingame.append_response("out", " ");
		}
		ingame.append_response("out", "\n");
	}
}

pub fn display_player_room(ingame: &mut MutIngame) {
	match room_of_player(ingame.ingame) {
		Ok(room) => display_room(ingame, room),
		Err(err) => ingame.append_response("err", &err.description())
	}
}

pub fn gen_display_current_room_action() -> core::Action {
	Box::new(|ingame, _| { display_player_room(ingame); Ok(()) } )
}

pub fn gen_move_actor_action(actor_ref: &Actor, direction: String) -> core::Action {
	let actor = actor_ref.clone();
	Box::new(move |mut ingame, _| {
		move_actor(ingame, &actor, &direction).is_ok();
		Ok(())
	})
}

pub fn gen_move_player_action(direction: String) -> core::Action {
	Box::new(move |mut ingame, _| {
		let player = get_player(ingame.ingame)?;
		move_actor(ingame, &player, &direction)
	})
}

pub fn gen_esge_package() -> EsgePackage {
	EsgePackage {
		init_action: Box::new(| _, _ | Ok(())),
		plausability_check: Box::new(| _ | None)
	}
}

pub fn insert_item_in_room<T: Itemizeable>(ingame: &mut MutIngame, 
										   item: Box<T>, 
										   mut player_room: Box<Room>) -> GameResult<()> {
	player_room.items.push(item.get_id().to_string());
	ingame.insert_item(player_room);
	ingame.insert_item(item);
	Ok(())
}

pub fn insert_item_in_player_room<T: Itemizeable>(ingame: &mut MutIngame, item: Box<T>) -> GameResult<()> {
	let player_room = room_of_player(ingame.ingame)?;
	insert_item_in_room(ingame, item, player_room)
}