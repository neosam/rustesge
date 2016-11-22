use core;
use core::Item;
use core::serialize_hashmap;
use core::deserialize_hashmap;
use core::serialize_vec;
use core::deserialize_vec;

use std::collections::HashMap;

pub struct Room {
	pub id: String,
	pub name: String,
	pub description: String,
	pub items: Vec<String>,
	pub actors: Vec<String>,
	pub exits: HashMap<String, String>
}

impl Room {
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
	pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
		self.name = name.into();
		self
	}
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
			let name: String = item.item_meta.get("name").cloned().unwrap_or(String::new());
			let desc = item.item_meta.get("desc").cloned().unwrap_or(String::new());
			let items_str = item.item_meta.get("items").cloned().unwrap_or(String::new());
			let items = deserialize_vec(&items_str);
			let actors_str = item.item_meta.get("actors").cloned().unwrap_or(String::new());
			let actors = deserialize_vec(&actors_str);
			let exits = deserialize_hashmap(&item.item_meta.get("exits").cloned().unwrap_or(String::new()));
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
		metas.insert("name".to_string(), self.name.clone());
		metas.insert("desc".to_string(), self.description.clone());
		metas.insert("items".to_string(), serialize_vec(&self.items));
		metas.insert("actors".to_string(), serialize_vec(&self.actors));
		metas.insert("exits".to_string(), serialize_hashmap(&self.exits));
	}
	fn get_id(&self) -> &str {
		&self.id
	}
}
