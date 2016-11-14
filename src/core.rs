use std::collections::HashMap;
use std::mem::swap;

pub struct Ingame {
	storage: Storage,
	actions: Actions,
	response: Response
}

pub struct MutIngame<'a> {
	pub ingame: &'a mut Ingame
}

pub struct Storage {
	items: HashMap<String, Item>
}
impl Storage {
	pub fn new() -> Self {
		Storage {
			items: HashMap::new()
		}
	}
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
	pub fn all_of_type<T>(&self) -> Vec<Box<T>> 
			where T: Itemizeable {
		self.items.values()
			.map(|x| T::from_item(x))
			.filter(|x| x.is_some())
			.map(|x| x.unwrap())
			.collect()
	}
}

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

#[derive(Clone, Debug)]
pub struct Item {
	pub item_type: String,
	pub item_id: String,
	pub item_meta: HashMap<String, String>
}
impl Item {
	pub fn new(item_type: String, item_id: String) -> Self {
		Item {
			item_type: item_type,
			item_id: item_id,
			item_meta: HashMap::new()
		}
	}
}

pub type Action = Box<Fn(&mut MutIngame, u32)>;

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

pub trait Itemizeable {
	fn from_item(item: &Item) -> Option<Box<Self>>;
	fn to_item(&self) -> Item;
	fn merge_into_item(&self, item: &mut Item);
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
	pub fn new() -> Self {
		Ingame {
			storage: Storage::new(),
			actions: Actions::new(),
			response: Response::new(),
		}
	}
	pub fn with_storage(storage: Storage) -> Self {
		Ingame {
			storage: storage,
			actions: Actions::new(),
			response: Response::new(),
		}
	}

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

	pub fn add_action(&mut self, action: Action) {
		self.actions.add_action(action)
	}
	pub fn add_one_time_action(&mut self, action: Action) {
		self.actions.add_one_time_action(action);
	}
	pub fn remove_action(&mut self, i: u32) {
		self.actions.remove_action(i);
	}

	pub fn get_response(&self, channel: &str) -> &str {
		self.response.get_response(channel)
	}
	pub fn get_item<T>(&self, item_id: &str) -> Option<Box<T>>
			where T: Itemizeable {
		self.storage.get_item(item_id)
	}
	pub fn all_of_type<T>(&self) -> Vec<Box<T>> 
			where T: Itemizeable {
		self.storage.all_of_type()
	}
}

impl<'a> MutIngame<'a> {
	pub fn insert_item<T>(&mut self, item: Box<T>)
			where T: Itemizeable {
		self.ingame.storage.insert(item)
	}
	pub fn get_item<T>(&self, item_id: &str) -> Option<Box<T>> 
			where T: Itemizeable {
		self.ingame.get_item(item_id)
	}

	pub fn add_action(&mut self, action: Action) {
		self.ingame.add_action(action)
	}
	pub fn remove_action(&mut self, i: u32) {
		self.ingame.remove_action(i);
	}	

	pub fn set_response(&mut self, channel: &str, msg: &str) {
		self.ingame.response.set_response(channel, msg)
	}

	pub fn get_response(&self, channel: &str) -> &str {
		self.ingame.get_response(channel)
	}

	pub fn append_response(&mut self, channel: &str, msg: &str) {
		self.ingame.response.append_response(channel, msg)
	}
}


pub fn serialize_vec(vec: &Vec<String>) -> String {
	vec.join(";").to_string()
}

pub fn deserialize_vec(string: &str) -> Vec<String> {
	string.split(";").map(|x| x.trim().to_string()).collect()
}


pub fn serialize_hashmap(map: &HashMap<String, String>) -> String {
	let mut res = String::new();
	let mut first = true;
	for (key, value) in map {
		if !first {
			res.push_str(";");
			first = false;
		}
		res.push_str(key);
		res.push_str(";");
		res.push_str(value);
	}
	res
}

pub fn deserialize_hashmap(string: &str) -> HashMap<String, String> {
	let mut split = string.split(";");
	let mut res = HashMap::new();
	loop {
		let key_option = split.next();
		let value_option = split.next();
		if key_option.is_none() || value_option.is_none() {
			break;
		}
		let key = key_option.unwrap().to_string();
		let value = value_option.unwrap().to_string();
		res.insert(key, value);
	}
	res
}


#[test]
fn simple_action_test() {
	let mut ingame = Ingame::new();
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
	let mut ingame = Ingame::new();
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
	let mut ingame = Ingame::new();
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


