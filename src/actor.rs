#![warn(missing_docs)]

//! Actors are somehow living creatures like the player or NPCs. 

use core;
use core::{Item, Meta};

use std::collections::HashMap;


/// An item which  represents a living creature in the world. 
#[derive(Clone)]
pub struct Actor {
	/// Internal identifier
	pub id: String,
	/// Name of the actor.
	pub name: String,
	/// More detailed description of the actor. 
	pub description: String
}

impl core::Itemizeable for Actor {
	fn from_item(item: &Item) -> Option<Box<Self>> {

		if item.item_type != "actor" {
			None
		} else {
			let name = item.meta_text_or_default("name", "").to_string();
			let desc = item.meta_text_or_default("desc", "").to_string();	
			Some(Box::new(Actor {
				id: item.item_id.clone(),
				name: name,
				description: desc
			}))
		}
	}
	fn to_item(&self) -> Item {
		let mut item = Item {
			item_id: self.id.clone(),
			item_type: "actor".to_string(),
			item_meta: HashMap::new()
		};
		self.merge_into_item(&mut item);
		item
	}
	fn merge_into_item(&self, item: &mut Item) {
		let metas = &mut item.item_meta;
		metas.insert("name".to_string(), Meta::Text(self.name.clone()));
		metas.insert("desc".to_string(), Meta::Text(self.description.clone()));
	}
	fn get_id(&self) -> &str {
		&self.id
	}
}
