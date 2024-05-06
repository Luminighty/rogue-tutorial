use std::cmp::{max, min};

use rltk::{RandomNumberGenerator, Rltk, Tile, RGB};
use specs::World;

use crate::utils::rect::Rect;

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;
pub const SIZE: usize = (WIDTH * HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
	pub tiles: Vec<TileType>,
	pub rooms: Vec<Rect>,
	pub width: i32,
	pub height: i32,
	pub revealed_tiles: Vec<bool>,
	pub visible_tiles : Vec<bool>
}

pub fn xy_idx(x: i32, y: i32) -> usize {
	(y as usize * 80) + x as usize
}

impl Map {

	fn new() -> Self {
		Self {
			tiles: vec![TileType::Wall; SIZE],
			rooms: Vec::new(),
			width: 80,
			height: 50,
			revealed_tiles: vec![false; SIZE],
			visible_tiles: vec![false; SIZE],
		}
	}

	fn add_room(&mut self, room: &Rect) {
		for y in room.y1+1..=room.y2 {
			for x in room.x1+1..=room.x2 {
				self.tiles[xy_idx(x, y)] = TileType::Floor;
			}
		}
	}

	fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
		for x in min(x1, x2)..=max(x1, x2) {
			let idx = xy_idx(x, y);
			if idx > 0 && idx < SIZE {
				self.tiles[idx as usize] = TileType::Floor;
			}
		}
	}
	
	fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
		for y in min(y1, y2)..=max(y1, y2) {
			let idx = xy_idx(x, y);
			if idx > 0 && idx < SIZE {
				self.tiles[idx as usize] = TileType::Floor;
			}
		}
	}

}

impl rltk::Algorithm2D for Map {
	fn dimensions(&self) -> rltk::prelude::Point {
		rltk::Point::new(self.width, self.height)
	}
}

impl rltk::BaseMap for Map {
	fn is_opaque(&self, idx: usize) -> bool {
		self.tiles[idx as usize] == TileType::Wall
	}
}



pub fn generate_map() -> Map {
	let mut map = Map::new();
	const MAX_ROOMS : i32 = 30;
	const MIN_SIZE : i32 = 6;
	const MAX_SIZE : i32 = 10;

	let mut rng = RandomNumberGenerator::new();

	for _ in 0..MAX_ROOMS {
		let w = rng.range(MIN_SIZE, MAX_SIZE);
		let h = rng.range(MIN_SIZE, MAX_SIZE);
		let x = rng.roll_dice(1, 80 - w - 1) - 1;
		let y = rng.roll_dice(1, 50 - h - 1) - 1;
		let new_room = Rect::new(x, y, w, h);

		let intersects = map.rooms.iter().any(|other| other.intersect(&new_room));
		if !intersects {
			map.add_room(&new_room);

			if !map.rooms.is_empty() {
				let (new_x, new_y) = new_room.center();
				let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
				if rng.range(0, 2) == 1 {
					map.add_horizontal_tunnel(prev_x, new_x, prev_y);
					map.add_vertical_tunnel(prev_y, new_y, new_x);
				} else {
					map.add_horizontal_tunnel(prev_x, new_x, new_y);
					map.add_vertical_tunnel(prev_y, new_y, prev_x);
				}
			}
			map.rooms.push(new_room);
		}
	}
	
	map
}

pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
	let map = ecs.fetch::<Map>();

	for (idx, tile) in map.tiles.iter().enumerate() {
		let x = idx % WIDTH as usize;
		let y = idx / WIDTH as usize;

		if !map.revealed_tiles[idx] { continue; }

		let (glyph, mut fg) = match tile {
			TileType::Floor => (
				rltk::to_cp437('.'),
				RGB::from_f32(0.0, 0.5, 0.5), 
			),
			TileType::Wall => (
				rltk::to_cp437('#'),
				RGB::from_f32(0.0, 1.0, 0.0),
			)
		};
		if !map.visible_tiles[idx] { fg = fg.to_greyscale(); }
		ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
	}
}
