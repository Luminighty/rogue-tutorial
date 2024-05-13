use specs::prelude::*;
use crate::state::State;


mod visibility;
mod monster;
mod map_indexing;
mod melee_combat;
mod item_pickup;
mod inventory_system;
pub mod damage;
pub mod player;
pub mod saveload_system;

macro_rules! run_now {
	($system: expr, $ecs: expr) => {
		let mut sys = $system;
		sys.run_now($ecs);
	};
}

pub fn run_systems(state: &mut State) {
	run_now!(visibility::VisibilitySystem {}, &state.ecs);
	run_now!(monster::MonsterAI {}, &state.ecs);
	run_now!(map_indexing::MapIndexingSystem {}, &state.ecs);
	run_now!(melee_combat::MeleeCombatSystem {}, &state.ecs);
	run_now!(damage::DamageSystem {}, &state.ecs);
	run_now!(item_pickup::ItemCollectionSystem {}, &state.ecs);
	run_now!(inventory_system::ItemUseSystem {}, &state.ecs);
	run_now!(inventory_system::ItemDropSystem {}, &state.ecs);

	state.ecs.maintain();
}
