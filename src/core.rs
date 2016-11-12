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
	actions: Vec<Action>
}
impl Actions {
    fn new() -> Self {
    	Actions {
    		actions: Vec::new()
    	}
    }
    fn with_capacity(len: usize) -> Self {
    	Actions {
    		actions: Vec::with_capacity(len)
    	}
    }
    fn len(&self) -> usize {
    	self.actions.len()
    }
    fn add_action(&mut self, action: Action) {
    	self.actions.push(action)
    }
}

#[derive(Clone, Debug)]
pub struct Item {
	pub item_type: String,
	pub item_id: String,
	pub item_meta: HashMap<String, String>
}

type Action = Box<Fn(&mut MutIngame)>;

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

	pub fn step(&mut self) {
		let mut actions = Actions::with_capacity(self.actions.len() * 2);
		self.response.clear();
		swap(&mut self.actions, &mut actions);		
		{
			let mut mutable_ingame = MutIngame { ingame: self };
			for action in actions.actions {
				action(&mut mutable_ingame);
			}
		}
	}

	pub fn add_action(&mut self, action: Action) {
		self.actions.add_action(action)
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
fn simple_test() {
	let mut ingame = Ingame::new();
	let action: Action = Box::new(|mut mut_ingame| mut_ingame.append_response("out", "test"));
	assert_eq!("", ingame.get_response("out"));
	ingame.add_action(action);
	assert_eq!("", ingame.get_response("out"));
	ingame.step();
	assert_eq!("test", ingame.get_response("out"));
	ingame.step();
	assert_eq!("", ingame.get_response("out"));
}