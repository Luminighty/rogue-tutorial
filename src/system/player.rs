use std::cmp;
use rltk::{console, Rltk, VirtualKeyCode};
use specs::{storage::GenericReadStorage, Entity, Join, World, WorldExt};

use crate::{component::*, resource::{gamelog::GameLog, map::{Map, TileType}, player::{PlayerData, PlayerEntity}}, state::{RunState, State}};


pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState  {
	// Player movement
	match ctx.key {
		None => { return RunState::AwaitingInput; } // Nothing happened
		Some(key) => match key {
			VirtualKeyCode::Left |
			VirtualKeyCode::Numpad4 |
			VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

			VirtualKeyCode::Right |
			VirtualKeyCode::Numpad6 |
			VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

			VirtualKeyCode::Up |
			VirtualKeyCode::Numpad8 |
			VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

			VirtualKeyCode::Down |
			VirtualKeyCode::Numpad2 |
			VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

			VirtualKeyCode::Numpad9 |
			VirtualKeyCode::Z => try_move_player(1, -1, &mut gs.ecs),

			VirtualKeyCode::Numpad7 |
			VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

			VirtualKeyCode::Numpad3 |
			VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

			VirtualKeyCode::Numpad1 |
			VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

			VirtualKeyCode::G => get_item(&mut gs.ecs),
			VirtualKeyCode::I => return RunState::ShowInventory,
			VirtualKeyCode::D => return RunState::ShowDropItem,

			VirtualKeyCode::Escape => return RunState::SaveGame,
			_ => return RunState::AwaitingInput
		},
	};
	RunState::PlayerTurn
}

fn get_item(ecs: &mut World) {
	let player_data = ecs.fetch::<PlayerData>();
	let player_entity = ecs.fetch::<PlayerEntity>();
	let entities = ecs.entities();
	let items = ecs.read_storage::<Item>();
	let positions = ecs.read_storage::<Position>();
	let mut gamelog = ecs.fetch_mut::<GameLog>();

	let mut target_item: Option<Entity> = None;
	for (item, _, position) in (&entities, &items, &positions).join() {
		if position.x == player_data.position.x && position.y == player_data.position.y {
			target_item = Some(item);
			break;
		}
	}

	match target_item {
		None => gamelog.entries.push("There is nothing here to pickup".to_string()),
		Some(item) => {
			let mut pickup = ecs.write_storage::<WantsToPickupItem>();
			pickup.insert(player_entity.0, WantsToPickupItem::new(player_entity.0, item))
				.expect("Unable to start item pickup");
		}
	}

}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
	let mut positions = ecs.write_storage::<Position>();
	let mut players = ecs.write_storage::<Player>();
	let mut viewsheds = ecs.write_storage::<Viewshed>();
	let combat_stats = ecs.read_storage::<CombatStats>();
	let entities = ecs.entities();
	let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

	let map = ecs.fetch::<Map>();

	for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
		let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

	for potential_target in map.tile_content[destination_idx].iter() {
		if let Some(target) = combat_stats.get(*potential_target) {
			wants_to_melee.insert(
				entity, 
				WantsToMelee { target: *potential_target }
			).expect("Add target failed");
			return;
		}
	}

		if map.blocked[destination_idx] {
			continue;
		}
		pos.x = cmp::min(79 , cmp::max(0, pos.x + delta_x));
		pos.y = cmp::min(49, cmp::max(0, pos.y + delta_y));

		let mut player_data = ecs.write_resource::<PlayerData>();
		player_data.position.x = pos.x;
		player_data.position.y = pos.y;

		viewshed.dirty = true;
	}
}
