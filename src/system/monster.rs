use rltk::{console, Point};
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{component::*, resource::{ map::Map, player::*}, state::RunState};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
	type SystemData = (
		WriteExpect<'a, Map>,
		ReadExpect<'a, PlayerData>,
		ReadExpect<'a, PlayerEntity>,
		ReadExpect<'a, RunState>,
		Entities<'a>,
		WriteStorage<'a, Viewshed>,
		ReadStorage<'a, Name>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Monster>,
		WriteStorage<'a, WantsToMelee>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			mut map,
			player_data,
			player,
			run_state,
			entities,
			mut viewshed, 
			name,
			mut position,
			monster,
			mut wants_to_melee
		) = data;

		if *run_state != RunState::MonsterTurn { return; }

		for (entity, viewshed, name, _monster, position) in (&entities, &mut viewshed, &name, &monster, &mut position).join() {
			let distance = rltk::DistanceAlg::Pythagoras.distance2d(
				Point::new(position.x, position.y), 
				Point::new(player_data.position.x, player_data.position.y)
			);
			if distance < 1.5 {
				wants_to_melee.insert(entity, WantsToMelee { target: player.0 }).expect("Unable to melee");
				return;
			}
			
			if !viewshed.visible_tiles.contains(&player_data.position) {
				continue;
			}
			let path = rltk::a_star_search(
				map.xy_idx(position.x, position.y), 
				map.xy_idx(player_data.position.x, player_data.position.y), 
				&mut *map
			);

			if path.success && path.steps.len() > 1 {
				let mut idx = map.xy_idx(position.x, position.y);
				map.blocked[idx] = false;
				position.x = path.steps[1] as i32 % map.width;				
				position.y = path.steps[1] as i32 / map.width;				
				idx = map.xy_idx(position.x, position.y);
				map.blocked[idx] = true;
				viewshed.dirty = true
			}
		}
	}
}

