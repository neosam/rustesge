#![warn(missing_docs)]

//! Adds rooms to the game structure.

use core;
use core::{Item, Meta};
use core::serialize_hashmap;
use core::deserialize_hashmap;

use std::collections::HashMap;

/// Introduces rooms to storageable objects.
#[derive(Clone, Debug)]
pub struct Room {
	/// The internal identifier.
	pub id: String,
	/// The name of the room.
	pub name: String,
	/// The description of the string.
	pub description: String,
	/// Item identifier of the room.
	pub items: Vec<String>,
	/// Actor identifier in the room.
	pub actors: Vec<String>,
	/// Exits and the identifier to a room id.
	pub exits: HashMap<String, String>
}

impl Room {
	/// Add a new room with the given id.
	pub fn new<S>(id: S) -> Self 
				where String: From<S> {
		Room {
			id: String::from(id),
			name: "".to_string(),
			description: "".to_string(),
			items: Vec::new(),
			actors: Vec::new(),
			exits: HashMap::new()
		}
	}

	/// Consumes the room and returns new one with the given name.
	///
	/// Use for initialization.
	pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
		self.name = name.into();
		self
	}

	/// Consumes the room and returns new one with the given description.
	///
	/// Use for initialization.
	pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
		self.description = description.into();
		self
	}
}

impl core::Itemizeable for Room {
	fn from_item(item: &Item) -> Option<Box<Room>> {
		if item.item_type != "room" {
			None
		} else {
			let name: String = item.meta_text_or_default("name", "").to_string();
			let desc = item.meta_text_or_default("desc", "").to_string();
			let items: Vec<String> = 
				item.meta_textvec_or_default("items", &[])
						.iter().map(|x| x.to_string()).collect();
			let actors: Vec<String> = 
				item.meta_textvec_or_default("actors", &[])
						.iter().map(|x| x.to_string()).collect();
			let exits = deserialize_hashmap(
					&item.meta_text_or_default("exits", "").to_string());
			Some(Box::new(Room {
				id: item.item_id.clone(),
				name: name,
				description: desc,
				items: items,
				actors: actors,
				exits: exits
			}))
		}
	}
	fn to_item(&self) -> Item {
		let mut item = Item {
			item_id: self.id.clone(),
			item_type: "room".to_string(),
			item_meta: HashMap::new()
		};
		self.merge_into_item(&mut item);
		item
	}
	fn merge_into_item(&self, item: &mut Item) {
		let metas = &mut item.item_meta;
		metas.insert("name".to_string(), Meta::Text(self.name.clone()));
		metas.insert("desc".to_string(), Meta::Text(self.description.clone()));
		metas.insert("items".to_string(), Meta::TextVec(self.items.clone()));
		metas.insert("actors".to_string(), Meta::TextVec(self.actors.clone()));
		metas.insert("exits".to_string(), Meta::Text(serialize_hashmap(&self.exits)));
	}
	fn get_id(&self) -> &str {
		&self.id
	}
}
