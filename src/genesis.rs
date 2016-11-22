//! Create a world inside another world


use core::{Storage};
use room::Room;
use actor::Actor;
use base::BaseGame;

/// Get ste minimal storage required
pub fn initial_genesis<S>(player_name: S) -> Storage 
				where S: Into<String> {
	let player = Actor {
		id: "player-actor".to_string(),
		name: player_name.into(),
		description: "You".to_string()
	};
	let base_game = BaseGame {
		player: "player-actor".to_string()
	};
	let room = Room::new("genesis-room")
				.with_name("Genesis");
	Storage::new("genesis-stor")
		.with_item(player)
		.with_item(base_game)
		.with_item(room)
}