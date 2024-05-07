use specs::{storage::GenericReadStorage, Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{component::{CombatStats, InBackpack, Name, Position, Potion, WantsToDrinkPotion, WantsToDropItem}, resource::{gamelog::GameLog, player::PlayerEntity}};

pub struct PotionUseSystem;

impl<'a> System<'a> for PotionUseSystem {
	type SystemData = (
		ReadExpect<'a, PlayerEntity>,
		WriteExpect<'a, GameLog>,
		Entities<'a>,
		WriteStorage<'a, WantsToDrinkPotion>,
		ReadStorage<'a, Name>,
		ReadStorage<'a, Potion>,
		WriteStorage<'a, CombatStats>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			player,
			mut gamelog,
			entities,
			mut wants_drink,
			names,
			potions,
			mut stats,
		) = data;

		for (entity, drink, stats) in (&entities, &wants_drink, &mut stats).join() {
			match potions.get(drink.potion) {
				None => {},
				Some(potion) => {
					stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
					if entity == player.0 {
						let name = Name::unwrap(names.get(drink.potion));
						gamelog.log(format!("You drink the {}, healing {}, hp", name, potion.heal_amount));
					}
					entities.delete(drink.potion).expect("Delete failed");
				}
			}
		}

		wants_drink.clear();
	}
}


pub struct ItemDropSystem;

impl<'a> System<'a> for ItemDropSystem {
	type SystemData = (
		ReadExpect<'a, PlayerEntity>,
		WriteExpect<'a, GameLog>,
		Entities<'a>,
		WriteStorage<'a, WantsToDropItem>,
		ReadStorage<'a, Name>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, InBackpack>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			player,
			mut gamelog,
			entities,
			mut wants_drop,
			names,
			mut positions,
			mut backpack
		) = data;

		for (entity, to_drop) in (&entities, &wants_drop).join() {
			let dropper_position = if let Some(position) = positions.get(entity) {
				position.clone()
			} else {
				Position::new(0, 0)
			};
			positions.insert(to_drop.item, dropper_position).expect("Failed to add position component");
			backpack.remove(to_drop.item);

			if entity == player.0 {
				let name = Name::unwrap(names.get(to_drop.item));
				gamelog.log(format!("You drop the {}.", name));
			}
		}
		wants_drop.clear();
	}
}