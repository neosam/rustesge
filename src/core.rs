//! Contains the core functionality required for a game.
//!
//! It provides the core elements like *Ingame*, *MutIngame*, *Action*
//! and the *Storage*.  Additionally it includes helper functions to deal with
//! these types.  Not included are interpreted game elements like Rooms or
//! Actors.

#![warn(missing_docs)]
use std::collections::HashMap;
use std::mem::swap;
use rustc_serialize::json;
use rustc_serialize::json::{EncoderError, DecoderError};

/// Contains the core game state with immutable storage.
///
/// It is not possible to modify the storage and the response but
/// *Actions* may be added or removed.
pub struct Ingame {
	storage: Storage,
	actions: Actions,
	response: Response
}

/// Provides mutable access to the 'Ingame' object.
///
/// It is meant to be passed only to an 'Action' as a mutable reference.
/// By this, it should make sure, that the game state can only be changed
/// internally by defined API calls. 
pub struct MutIngame<'a> {
	/// Mutable reference to the Ingame to access it directly.
	pub ingame: &'a mut Ingame
}

/// Contains all items required for a game (Room, Actor). 
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Storage {
	id: String,
	items: HashMap<String, Item>
}
impl Storage {
	/// Generate a new and empty 'Storage'.
	pub fn new(id: String) -> Self {
		Storage {
			id: id,
			items: HashMap::new()
		}
	}

	/// Insert an item to the Storage.
	pub fn insert<T>(&mut self, item: Box<T>) 
			where T: Itemizeable {
		let item_id = item.get_id();
		if self.items.contains_key(item_id) {
			let mut stored_item = self.items.get_mut(item_id).unwrap();
			item.merge_into_item(stored_item);
		} else {
			let item = item.to_item();
			self.items.insert(item_id.to_string(), item);
		}
	}

	/// Get in item from the storage.
	///
	/// # Errors
	/// It will return None if the item was not found or if it cannot be
	/// converted to T.
	pub fn get_item<T>(&self, item_id: &str) -> Option<Box<T>>
			where T: Itemizeable {
		let item_option = self.items.get(item_id);
		if item_option.is_none() {
			None
		} else {
			let item = item_option.unwrap();
			T::from_item(item)
		}
	}

	/// Return a list of all item which can be converted to type T.
	pub fn all_of_type<'a, T>(&'a self) -> Box<Iterator<Item=Box<T>> + 'a>
			where T: Itemizeable {
		Box::new(self.items.values()
			.map(|x| T::from_item(x))
			.filter(|x| x.is_some())
			.map(|x| x.unwrap()))
	}
}

/// Holds all actions in an Ingame object.
pub struct Actions {
	actions: HashMap<u32, Action>,
	new_actions: Vec<(u32, Action)>,
	delete_actions: Vec<u32>,
	one_time_actions: Vec<Action>,
	index: u32
}
impl Actions {
    fn new() -> Self {
    	Actions {
    		actions: HashMap::new(),
    		new_actions: Vec::new(),
    		delete_actions: Vec::new(),
    		one_time_actions: Vec::new(),
    		index: 0
    	}
    }
    fn add_action(&mut self, action: Action) {
    	self.index += 1;
    	self.new_actions.push((self.index, action));
    }
    fn add_one_time_action(&mut self, action: Action) {
    	self.one_time_actions.push(action);
    }
    fn apply_actions(&mut self) {
    	if !self.new_actions.is_empty() {
    		let mut new_actions: Vec<(u32, Action)> = Vec::new();
    		swap(&mut new_actions, &mut self.new_actions);
    		for (i, a) in new_actions {
    			self.actions.insert(i, a);
    		}
    	}
    	if !self.delete_actions.is_empty() {
    		let mut delete_actions: Vec<u32> = Vec::new();
    		swap(&mut delete_actions, &mut self.delete_actions);
    		for i in delete_actions {
    			self.actions.remove(&i);
    		}
    	}
    }
    fn remove_action(&mut self, index: u32) {
    	self.delete_actions.push(index);
    }

}

/// Any item in a game (Room, Actor, Money, the game state)
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Item {
	/// ID of the type ("room", "actor")
	pub item_type: String,
	
	/// Unique ID of the object.
	pub item_id: String,

	/// Additional information like name and description.
	pub item_meta: HashMap<String, String>
}
impl Item {
	/// New item with given type and item ID.
	pub fn new(item_type: String, item_id: String) -> Self {
		Item {
			item_type: item_type,
			item_id: item_id,
			item_meta: HashMap::new()
		}
	}
}

/// Definition of an Action.
pub type Action = Box<Fn(&mut MutIngame, u32)>;

/// Holds the responses from the actions.
pub struct Response {
	items: HashMap<String, String>
}
impl Response {
	fn new() -> Self {
		Response {
			items: HashMap::new()
		}
	}
	fn set_response(&mut self, channel: &str, msg: &str) {
		self.items.insert(channel.to_string(), msg.to_string());
	}
	fn get_response(&self, channel: &str) -> &str {
		if self.items.contains_key(channel) {
			self.items.get(channel).unwrap()
		} else {
			""
		}
	}
	fn append_response(&mut self, channel: &str, msg: &str) {
		if self.items.contains_key(channel) {
			self.items.get_mut(channel).unwrap().push_str(msg);
		} else {
			self.set_response(channel, msg);
		}
	}
	fn clear(&mut self) {
		self.items.clear()
	}
}

/// Enables structs to be inserted in the Ingame storage.
pub trait Itemizeable {
	/// Convert in Item to the struct.
	///
	/// # Errors
	/// Return None if not compatible.
	fn from_item(item: &Item) -> Option<Box<Self>>;

	/// Converts the Struct into the Item
	fn to_item(&self) -> Item;

	/// Merges itself into the given item.
	///
	/// May overwrite some metas. 
	fn merge_into_item(&self, item: &mut Item);

	/// Get the ID if itself.
	fn get_id(&self) -> &str;
}
impl Itemizeable for Item {
	fn from_item(item: &Item) -> Option<Box<Item>> {
		Some(Box::new(item.clone()))
	}
	fn to_item(&self) -> Item {
		self.clone()
	}
	fn merge_into_item(&self, item: &mut Item) {
		for (key, value) in self.item_meta.iter() {
			item.item_meta.insert(key.clone(), value.clone());
		}
	}
	fn get_id(&self) -> &str { &self.item_id }
}


impl Ingame {
	/// Create new Ingame without items and actions.
	pub fn new(id: String) -> Self {
		Ingame {
			storage: Storage::new(id),
			actions: Actions::new(),
			response: Response::new(),
		}
	}

	/// Create a new Ingame with the consumed Storage.
	pub fn with_storage(storage: Storage) -> Self {
		Ingame {
			storage: storage,
			actions: Actions::new(),
			response: Response::new(),
		}
	}

	/// Performs one game step.  Basically runs the Actions.
	pub fn step(&mut self) {
		self.response.clear();
		self.actions.apply_actions();
		{
			let mut actions: HashMap<u32, Action> = HashMap::new();
			swap(&mut actions, &mut self.actions.actions);
			{
				let mut mutable_ingame = MutIngame { ingame: self };
				for (i, action) in actions.iter() {
					action(&mut mutable_ingame, *i);
				}
			}
			swap(&mut actions, &mut self.actions.actions);
		}
		{
			let mut actions: Vec<Action> = Vec::new();
			swap(&mut actions, &mut self.actions.one_time_actions);
			for action in actions {
				let mut mutable_ingame = MutIngame { ingame: self };
				action(&mut mutable_ingame, 0)
			}
		}
	}

	/// Add a new action.
	pub fn add_action(&mut self, action: Action) {
		self.actions.add_action(action)
	}

	/// Add a action which is only run once on the next step.
	pub fn add_one_time_action(&mut self, action: Action) {
		self.actions.add_one_time_action(action);
	}

	/// Remove the action with the given index.
	pub fn remove_action(&mut self, i: u32) {
		self.actions.remove_action(i);
	}

	/// Read the response of the given channel.
	pub fn get_response(&self, channel: &str) -> &str {
		self.response.get_response(channel)
	}

	/// Get an item from the storage.
	pub fn get_item<T>(&self, item_id: &str) -> Option<Box<T>>
			where T: Itemizeable {
		self.storage.get_item(item_id)
	}

	/// Get all items which can be converted to T.
	pub fn all_of_type<'a, T>(&'a self) -> Box<Iterator<Item=Box<T>> + 'a>
			where T: Itemizeable {
		self.storage.all_of_type()
	}


	/// Transform storage to JSON string.
	pub fn serialize(&self) -> Result<String, EncoderError> {
		json::encode(&self.storage)
	}

	/// Creates an ingame with the storage defined in the JSON string.
	pub fn from_json(msg: &str) -> Result<Self, DecoderError> {
		let storage: Storage = try!(json::decode(msg));
		Ok(Ingame::with_storage(storage))
	}
}

impl<'a> MutIngame<'a> {
	/// Insert or replace an item.
	pub fn insert_item<T>(&mut self, item: Box<T>)
			where T: Itemizeable {
		self.ingame.storage.insert(item)
	}

	/// Get in item.
	///
	/// # Errors
	/// None if the type was not found or could not be converted.
	pub fn get_item<T>(&self, item_id: &str) -> Option<Box<T>> 
			where T: Itemizeable {
		self.ingame.get_item(item_id)
	}

	/// Add an action.
	pub fn add_action(&mut self, action: Action) {
		self.ingame.add_action(action)
	}

	/// Add a action which is only run once on the next step.
	pub fn add_one_time_action(&mut self, action: Action) {
		self.ingame.add_one_time_action(action);
	}	

	/// Remove the action of the given id.
	pub fn remove_action(&mut self, i: u32) {
		self.ingame.remove_action(i);
	}	

	/// Overwrite or set the response at the given channel.
	pub fn set_response(&mut self, channel: &str, msg: &str) {
		self.ingame.response.set_response(channel, msg)
	}

	/// Get the response of the given channel.
	pub fn get_response(&self, channel: &str) -> &str {
		self.ingame.get_response(channel)
	}

	/// Append the string slice to the end of the given channel.
	pub fn append_response(&mut self, channel: &str, msg: &str) {
		self.ingame.response.append_response(channel, msg)
	}
}

/// Turn a Vec of Strings into a semicolon separated String.
pub fn serialize_vec(vec: &Vec<String>) -> String {
	vec.join(";").to_string()
}

/// Turn a String of semicolon separated items into a Vec of Strings.
pub fn deserialize_vec(string: &str) -> Vec<String> {
	string.split(";").map(|x| x.trim().to_string()).collect()
}

/// Turn a HashMap of Strings into a String with semicolon as separator.
pub fn serialize_hashmap(map: &HashMap<String, String>) -> String {
	json::encode(map).unwrap()
}

/// Turn a String separated by semicolon to a HashMap of Strings.
pub fn deserialize_hashmap(string: &str) -> HashMap<String, String> {
	json::decode(string).unwrap()
}


impl Itemizeable for Storage {
	fn from_item(item: &Item) -> Option<Box<Self>> {
		if &item.item_type != "storage" {
			None
		} else {
			if let Some(item_map) = item.item_meta.get("items") {
				if let Ok(items) = json::decode(&item_map) {
					let storage = Storage {
						id: item.item_id.clone(),
						items: items
					};
					Some(Box::new(storage))
				} else {
					None
				}
			} else {
				None
			}
		}
	}

	fn to_item(&self) -> Item {
		let mut items = Item {
			item_id: self.id.clone(),
			item_type: "storage".to_string(),
			item_meta: HashMap::new()
		};
		self.merge_into_item(&mut items);
		items
	}

	fn merge_into_item(&self, item: &mut Item) {
		let items = json::encode(&self.items).unwrap();
		item.item_meta.insert("items".to_string(), items);
	}

	fn get_id(&self) -> &str {
		&self.id
	}
}

#[test]
fn simple_action_test() {
	let mut ingame = Ingame::new("storage");
	let action: Action = Box::new(|mut mut_ingame, _| mut_ingame.append_response("out", "test"));
	assert_eq!("", ingame.get_response("out"));
	ingame.add_action(action);
	assert_eq!("", ingame.get_response("out"));
	ingame.step();
	assert_eq!("test", ingame.get_response("out"));
	ingame.step();
	assert_eq!("test", ingame.get_response("out"));
}

#[test]
fn remove_action_test() {
	let mut ingame = Ingame::new("storage");
	let action: Action = Box::new(|mut mut_ingame, i| {
		mut_ingame.remove_action(i);
		mut_ingame.append_response("out", "test");
	});
	assert_eq!("", ingame.get_response("out"));
	ingame.add_action(action);
	assert_eq!("", ingame.get_response("out"));
	ingame.step();
	assert_eq!("test", ingame.get_response("out"));
	ingame.step();
	assert_eq!("", ingame.get_response("out"));
}

#[test]
fn one_time_action_test() {
	let mut ingame = Ingame::new("storage");
	let action: Action = Box::new(|mut mut_ingame, _| {
		mut_ingame.append_response("out", "test");
	});
	assert_eq!("", ingame.get_response("out"));
	ingame.add_one_time_action(action);
	assert_eq!("", ingame.get_response("out"));
	ingame.step();
	assert_eq!("test", ingame.get_response("out"));
	ingame.step();
	assert_eq!("", ingame.get_response("out"));
}


