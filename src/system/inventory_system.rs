use rltk::console;
use specs::{storage::GenericReadStorage, Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{component::{AreaOfEffect, CombatStats, Confusion, Consumable, InBackpack, InflictsDamage, Name, Position, ProvidesHealing, SufferDamage, WantsToDropItem, WantsToUseItem}, resource::{gamelog::GameLog, map::Map, player::PlayerEntity}};

pub struct ItemUseSystem;

impl<'a> System<'a> for ItemUseSystem {
	type SystemData = (
		ReadExpect<'a, PlayerEntity>,
		ReadExpect<'a, Map>,
		WriteExpect<'a, GameLog>,
		Entities<'a>,
		WriteStorage<'a, WantsToUseItem>,
		ReadStorage<'a, Name>,
		ReadStorage<'a, ProvidesHealing>,
		ReadStorage<'a, InflictsDamage>,
		ReadStorage<'a, Consumable>,
		ReadStorage<'a, AreaOfEffect>,
		WriteStorage<'a, Confusion>,
		WriteStorage<'a, CombatStats>,
		WriteStorage<'a, SufferDamage>,
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			player,
			map,
			mut gamelog,
			entities,
			mut wants_drink,
			names,
			healing,
			damages,
			consumables,
			aoe,
			mut confusion,
			mut combat_stats,
			mut suffer_damage,
		) = data;

		for (entity, use_item) in (&entities, &wants_drink).join() {

			let mut targets = Vec::new();
			match use_item.target {
				None => { targets.push(player.0); }
				Some(target) => {
					match aoe.get(use_item.item) {
						None => {
							let idx = map.xy_idx(target.x, target.y);
							for mob in map.tile_content[idx].iter() {
								targets.push(*mob);
							}
						},
						Some(aoe) => {
							let mut blast_tiles = rltk::field_of_view(target, aoe.radius, &*map);
							blast_tiles.retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);
							for tile_idx in blast_tiles.iter() {
								let idx = map.xy_idx(tile_idx.x, tile_idx.y);
								for mob in map.tile_content[idx].iter() {
									targets.push(*mob);
								}
							}
						}
					}
				}
			}

			if let Some(healing) = healing.get(use_item.item) {
				for target in targets.iter() {
					if let Some(stats) = combat_stats.get_mut(*target) {
						stats.hp = i32::min(stats.max_hp, stats.hp + healing.heal_amount);
						if entity == player.0 {
							let name = Name::unwrap(names.get(use_item.item));
							gamelog.log(format!("You drink the {}, healing {}, hp", name, healing.heal_amount));
						}
					}
				}
			}


			if let Some(damage) = damages.get(use_item.item) {
				for target in targets.iter() {
					SufferDamage::new_damage(&mut suffer_damage, *target, damage.damage);
					if entity == player.0 {
						let mob_name = Name::unwrap(names.get(*target));
						let item_name = Name::unwrap(names.get(use_item.item));
						gamelog.log(format!("You use {} on {}, inflicting {} hp.", item_name, mob_name, damage.damage));
					}
				}
			}

			{
				let mut add_confusion = Vec::new();
				if let Some(confusion) = confusion.get(use_item.item) {
					for mob in targets.iter() {
						add_confusion.push((*mob, confusion.turns));
						if entity == player.0 {
							let name = Name::unwrap(names.get(*mob));
							let item = Name::unwrap(names.get(use_item.item));
							gamelog.log(format!("You use {} on {}, confusing them.", item, name));
						}
					}
				}
				for (mob, turns) in add_confusion {
					confusion.insert(mob, Confusion { turns })
						.expect("Unable to insert confusion");
				}
			}
			if consumables.contains(use_item.item) {
				entities.delete(use_item.item).expect("Delete failed");				
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