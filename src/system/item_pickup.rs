use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{component::*, resource::{gamelog::{self, GameLog}, player::PlayerEntity}};

pub struct ItemCollectionSystem;

impl <'a> System<'a> for ItemCollectionSystem {
	type SystemData = (
		ReadExpect<'a, PlayerEntity>,
		WriteExpect<'a, GameLog>,
		WriteStorage<'a, WantsToPickupItem>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Name>,
		WriteStorage<'a, InBackpack>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			player, 
			mut game_log, 
			mut wants_to_pickup, 
			mut position,
			name,
			mut backpack
		) = data;

		for pickup in wants_to_pickup.join() {
			position.remove(pickup.item);
			backpack.insert(pickup.item, InBackpack::new(pickup.collected_by)).expect("Failed to insert into backpack");

			if pickup.collected_by == player.0 {
				let name = name.get(pickup.item).map(|name| name.name.to_string()).unwrap_or("???".to_string());
				game_log.log(format!("You pick up the {}.", name));
			}
		}

		wants_to_pickup.clear();
	}
}