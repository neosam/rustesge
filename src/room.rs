use core;
use core::Item;
use core::serialize_hashmap;
use core::deserialize_hashmap;
use core::serialize_vec;
use core::deserialize_vec;

use std::collections::HashMap;

struct Room {
	id: String,
	name: String,
	description: String,
	items: Vec<String>,
	individuals: Vec<String>,
	exits: HashMap<String, String>
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
			let ind_str = item.item_meta.get("ind").cloned().unwrap_or(String::new());
			let ind = deserialize_vec(&ind_str);
			let exits = deserialize_hashmap(&item.item_meta.get("exits").cloned().unwrap_or(String::new()));
			Some(Box::new(Room {
				id: item.item_id.clone(),
				name: name,
				description: desc,
				items: items,
				individuals: ind,
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
		metas.insert("ind".to_string(), serialize_vec(&self.individuals));
		metas.insert("exits".to_string(), serialize_hashmap(&self.exits));
	}
	fn get_id(&self) -> &str {
		&self.id
	}
}