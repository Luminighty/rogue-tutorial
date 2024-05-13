use rltk::console;
use specs::{Entity, Join, System, World, WorldExt, WriteStorage};

use crate::{component::{CombatStats, Name, Player, SufferDamage}, resource::gamelog};

pub struct DamageSystem;

impl<'a> System<'a> for DamageSystem {
	type SystemData = (
		WriteStorage<'a, CombatStats>,
		WriteStorage<'a, SufferDamage>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (mut stats, mut damage) = data;
		for (stats, damage) in (&mut stats, &damage).join() {
			stats.hp -= damage.amount.iter().sum::<i32>();
		}
		damage.clear();
	}
}

impl DamageSystem {
	pub fn delete_the_dead(ecs: &mut World) -> bool {
		let mut player_died = false;
		let mut dead: Vec<Entity> = Vec::new();
		{
			let combat_stats = ecs.read_storage::<CombatStats>();
			let entities = ecs.entities();
			let players = ecs.read_storage::<Player>();
			let mut game_log = ecs.fetch_mut::<gamelog::GameLog>();
			let names = ecs.read_storage::<Name>();
			for (entity, stats) in (&entities, &combat_stats).join() {
				if stats.hp > 0 { continue; }
				if let Some(_) = players.get(entity) { 
					console::log("you are dead.");
					player_died = true;
					continue; 
				}
				if let Some(name) = names.get(entity) {
					game_log.log(format!("{} died.", name.name));
				}
				dead.push(entity);
			}
		}
		ecs.delete_entities(&dead).expect("Unable to delete.");
		player_died
	}
}