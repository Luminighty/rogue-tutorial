use specs::prelude::*;
use crate::state::State;

mod visibility;
mod monster;
pub mod player;

pub fn run_systems(state: &mut State) {
	let mut visibility = visibility::VisibilitySystem {};
	visibility.run_now(&state.ecs);

	let mut mob = monster::MonsterAI {};
	mob.run_now(&state.ecs);

	state.ecs.maintain();
}
