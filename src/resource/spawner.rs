use specs::{World, WorldExt};

use crate::{templates, utils::rect::Rect};

use super::map::Map;

pub const MAX_MONSTERS: i32 = 4;
pub const MAX_ITEMS: i32 = 2;

pub fn spawn_room(ecs: &mut World, room: &Rect) {
	let width = { ecs.fetch::<Map>().width as usize };

	let monsters = {
		let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
		let monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
		select_points(monsters, &mut rng, room, width)
	};
	
	let items = {
		let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
		let items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;
		select_points(items, &mut rng, room, width)
	};
	

	for idx in monsters.iter() {
		let x = *idx % width;
		let y = *idx / width;
		templates::create_monster(ecs, x as i32, y as i32);
	}
	for idx in items.iter() {
		let x = *idx % width;
		let y = *idx / width;
		templates::item::health_potion(ecs, x as i32, y as i32);
	}
}

pub fn spawn_item(ecs: &mut World, room: &Rect) {
	let width = { ecs.fetch::<Map>().width as usize };
	let items = {
		let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
		select_points(1, &mut rng, room, width)
	};

	for idx in items.iter() {
		let x = *idx % width;
		let y = *idx / width;
		templates::item::health_potion(ecs, x as i32, y as i32);
	}
}

fn select_points(amount: i32, rng: &mut rltk::RandomNumberGenerator, room: &Rect, map_width: usize) -> Vec<usize> {
	let mut points = Vec::new();

	for _ in 0..amount {
		let mut added = false;
		while !added {
			let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
			let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
			let idx = (y * map_width) + x;
			if !points.contains(&idx) {
				points.push(idx);
				added = true;
			}
		}
	}

	points
}
