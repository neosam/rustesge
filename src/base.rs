#![warn(missing_docs)]

//! Combines important elements to get everything for a basic game.
//! This incudes: Actors, Rooms, Ingame setup.

use core;
use core::{Ingame, MutIngame, GameResult, GameError, Itemizeable, Meta, 
				gerr, berr};
use core::Item;
use actor::Actor;
use room::Room;

use std::result::Result;

/// Checks if the internal structure contains logical errors.
pub type PlausabilityCheck = Box<Fn(&Ingame) -> Option<String>>;

/// A package which can be used to initialize the game engine.
pub struct EsgePackage {
	init_action: core::Action,
	plausability_check: PlausabilityCheck
}

impl Ingame {
	/// Get the room of the actor given actor.
	///
	/// # Failure
	/// Returns an error if a room was not found.
	pub fn room_of_actor(&self, actor: &Actor) -> GameResult<Box<Room>> {
		let id = &actor.id;
		let rooms = self.all_of_type::<Room>();
		for room in rooms {
			if room.actors.contains(id) {
				return Ok(room)
			}
		}
		Err(berr("Room not found"))
	}

	/// Get a tuple of exit names and the rooms behind them. 
	pub fn exits_in_room<'a>(&'a self, room: &'a Room) 
							-> Box<Iterator<Item=(String, Box<Room>)> + 'a> {
		Box::new(room.exits.iter()
			.map(move|(label, room_id)| (label.to_string(), self.get_item(room_id)))
			.filter(| &(_, ref room_option) | room_option.is_some())
			.map(| (label, room_option) | (label, room_option.unwrap()))
		)
	}


	/// Git all items from a room.
	///
	/// Translates the IDs to the Box.
	pub fn items_in_room<'a>(&'a self, room: &'a Room) -> 
				Box<Iterator<Item=Box<Item>> + 'a> {
		Box::new(room.items.iter()
			.map(move|x| self.get_item(x))
			.filter(|x| x.is_some())
			.map(|x| x.unwrap())
		)
	}


	/// Get all actors from a room.
	pub fn actors_in_room<'a>(&'a self, room: &'a Room) -> 
								Box<Iterator<Item=Box<Actor>> + 'a> {
		Box::new(room.actors.iter()
			.map(move |x| self.get_item(x))
			.filter(|x| x.is_some())
			.map(|x| x.unwrap()))
	}

	/// Create an ingame regarding the storage and the package.
	pub fn init_packages(storage: core::Storage, 
						 packages: Vec<EsgePackage>) -> Result<Self, String> {
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


	/// Return the player of the ingme.
	pub fn get_player(&self) -> GameResult<Box<Actor>> {
		let base_game = self.get_item::<BaseGame>("base_game")
				.ok_or(GameError::new("Could not find base_game"))?;
		self.get_item::<Actor>(&base_game.player)
				.ok_or(berr(format!("Player not found: {}", base_game.player)))
	}

	/// Get the room which holds the player.
	pub fn room_of_player(&self) -> GameResult<Box<Room>> {
		let player = self.get_player()?;
		self.room_of_actor(&*player)
	}

}







impl<'a> MutIngame<'a> {
/// Remove the given actor from the given room.
	pub fn remove_actor_from_room(&mut self, actor: &Actor, mut room: Box<Room>) {
		let id = &actor.id;
		room.actors = room.actors.iter()
			.map(|x| x.to_string())
			.filter(|x| x != id)
			.collect();
		self.insert_item(room);
	}

	/// Move an actor to another room.
	///
	/// If the actor was not in a room yet it will just be inserted.
	pub fn warp_actor(&mut self, actor: &Actor, mut room: Box<Room>) {
		if let Ok(src_room) = self.ingame.room_of_actor(actor) {
			self.remove_actor_from_room(actor, src_room);
		}
		room.actors.push(actor.id.clone());
		self.insert_item(room);
	}

	/// Move an actor through a exit to another room.
	///
	/// # Failure
	/// Error if the room behind the exit was not found and if the exit is not
	/// in the room.
	pub fn move_actor(&mut self, 
					  actor: &Actor, 
					  exit_name: &str) -> GameResult<()> {
		let actor_room = self.ingame.room_of_actor(actor)?;
		let dest_room_name = actor_room.exits.get(exit_name)
				.ok_or(gerr("Could not find exit"))?;
		let dest_room: Box<Room> = self.get_item(dest_room_name)
				.ok_or(gerr("Could not get the destination room"))?;
		self.warp_actor(actor, dest_room.clone());
		self.display_room(dest_room);
		Ok(())
	}

	/// Print a room to out.
	pub fn display_room(&mut self, room: Box<Room>) {
		self.append_response("out", "Room: ");
		self.append_response("out", &room.name);
		self.append_response("out", "\n");
		self.append_response("out", &room.description);
		self.append_response("out", "\n");
		if !room.items.is_empty() {
			self.append_response("out", "Items: ");
			for item in &room.items {
				self.append_response("out", &item);
				self.append_response("out", " ");
			}
			self.append_response("out", "\n");
		}
		if !room.exits.is_empty() {
			self.append_response("out", "Directions: ");
			let keys = room.exits.keys();
			for key in keys {
				self.append_response("out", key);
				self.append_response("out", " ");
			}
			self.append_response("out", "\n");
		}
	}

	/// Display the room which holds the player.
	pub fn display_player_room(&mut self) -> GameResult<()> {
		let room = self.ingame.room_of_player()?;
		self.display_room(room);
		Ok(())
	}

	/// Insert an item into a room.
	///
	/// If inserts the item again and will replace the old one if it's already
	/// exists.
	pub fn insert_item_in_room<T: Itemizeable>(&mut self, 
											   item: Box<T>, 
											   mut player_room: Box<Room>) -> GameResult<()> {
		player_room.items.push(item.get_id().to_string());
		self.insert_item(player_room);
		self.insert_item(item);
		Ok(())
	}



	/// Insert item into the room which holds the player. 
	pub fn insert_item_in_player_room<T: Itemizeable>(&mut self, 
										item: Box<T>) -> GameResult<()> {
		let player_room = self.ingame.room_of_player()?;
		self.insert_item_in_room(item, player_room)
	}
}










/// Holds relevant information.
pub struct BaseGame {
	/// ID of the player which must be an actor.
	pub player: String
}

impl core::Itemizeable for BaseGame {
	fn from_item(item: &Item) -> Option<Box<Self>> {
		Some(Box::new(BaseGame {
			player: item.meta_text_or_default("player", "").to_string()
		}))
	}
	fn to_item(&self) -> core::Item {
		let mut item = core::Item::new("base_game".to_string(), "base_game".to_string());
		self.merge_into_item(&mut item);
		item
	}
	fn merge_into_item(&self, item: &mut Item) {
		item.item_meta.insert("player".to_string(), 
					Meta::Text(self.player.clone()));
	}
	fn get_id(&self) -> &str {
		"base_game"
	}
}






/// Creates an Action which displays the room which holds the player.
pub fn gen_display_current_room_action() -> core::Action {
	Box::new(|ingame, _| { ingame.display_player_room()?; Ok(()) } )
}

/// Creates an Action which moves an actor through an exit into another room.
pub fn gen_move_actor_action(actor_ref: &Actor, 
							 direction: String) -> core::Action {
	let actor = actor_ref.clone();
	Box::new(move |mut ingame, _| {
		ingame.move_actor(&actor, &direction).is_ok();
		Ok(())
	})
}

/// Creates an Action which moves the player through an exit to another room.
pub fn gen_move_player_action(direction: String) -> core::Action {
	Box::new(move |mut ingame, _| {
		let player = ingame.ingame.get_player()?;
		ingame.move_actor(&player, &direction)
	})
}

/// Create the base package.
pub fn gen_esge_package() -> EsgePackage {
	EsgePackage {
		init_action: Box::new(| _, _ | Ok(())),
		plausability_check: Box::new(| _ | None)
	}
}

