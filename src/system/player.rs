use std::cmp;
use rltk::{Rltk, VirtualKeyCode};
use specs::{Join, World, WorldExt};

use crate::{component::*, resource::map::{xy_idx, Map, TileType}, state::{RunState, State}};


pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState  {
	// Player movement
	match ctx.key {
		None => { return RunState::Paused; } // Nothing happened
		Some(key) => match key {
			VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
			VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
			VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
			VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
			VirtualKeyCode::Escape => { ctx.quit(); }
			_ => { return RunState::Paused; }
		},
	};
	RunState::Running
}


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
	let mut positions = ecs.write_storage::<Position>();
	let mut players = ecs.write_storage::<Player>();
	let mut viewsheds = ecs.write_storage::<Viewshed>();
	let map = ecs.fetch::<Map>();

	for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
		let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
		if map.tiles[destination_idx] == TileType::Wall {
			continue;
		}
		pos.x = cmp::min(79 , cmp::max(0, pos.x + delta_x));
		pos.y = cmp::min(49, cmp::max(0, pos.y + delta_y));

		viewshed.dirty = true;
	}
}
