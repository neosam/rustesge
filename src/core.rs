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
	pub fn insert(&mut self, item: Item) {
		self.items.insert(item.item_id.clone(), item);
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

pub struct Item {
	item_type: String,
	item_id: String,
	item_meta: HashMap<String, String>
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
	fn from_item(item: &Item) -> Self;
	fn to_item(&self) -> Item;
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
}

impl<'a> MutIngame<'a> {
	pub fn insert_item<T>(&mut self, item: &T)
			where T: Itemizeable {
		self.ingame.storage.insert(item.to_item())
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