use std::cmp::{max, min};

use rltk::{Point, RandomNumberGenerator, Rltk, Tile, RGB};
use serde::{Deserialize, Serialize};
use specs::{Entity, World};

use crate::utils::rect::Rect;

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 43;
pub const SIZE: usize = (WIDTH * HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    fn new(depth: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; SIZE],
            rooms: Vec::new(),
            width: WIDTH,
            height: HEIGHT,
            revealed_tiles: vec![false; SIZE],
            visible_tiles: vec![false; SIZE],
            blocked: vec![false; SIZE],
            tile_content: vec![Vec::new(); SIZE],
            depth,
        }
    }

    fn add_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < SIZE {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < SIZE {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let index = self.xy_idx(x, y);
        !self.blocked[index]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
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

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, index: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = index as i32 % self.width;
        let y = index as i32 / self.width;
        let w = self.width as usize;

        if self.is_exit_valid(x - 1, y) {
            exits.push((index - 1, 1.0));
        }
        if self.is_exit_valid(x + 1, y) {
            exits.push((index + 1, 1.0));
        }
        if self.is_exit_valid(x, y - 1) {
            exits.push((index - w, 1.0))
        }
        if self.is_exit_valid(x, y + 1) {
            exits.push((index + w, 1.0));
        }

        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((index - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((index - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((index + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((index + w) + 1, 1.45));
        }

        exits
    }
}

pub fn generate_map(depth: i32) -> Map {
    let mut map = Map::new(depth);
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let intersects = map.rooms.iter().any(|other| other.intersect(&new_room));
        if !intersects {
            map.add_room(&new_room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
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

    let stairs_position = map.rooms[map.rooms.len() - 1].center();
    let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
    map.tiles[stairs_idx] = TileType::DownStairs;

    map
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for (idx, tile) in map.tiles.iter().enumerate() {
        let x = idx % WIDTH as usize;
        let y = idx / WIDTH as usize;

        if !map.revealed_tiles[idx] {
            continue;
        }

        let (glyph, mut fg) = match tile {
            TileType::Floor => (rltk::to_cp437('.'), RGB::from_f32(0.0, 0.5, 0.5)),
            TileType::Wall => (rltk::to_cp437('#'), RGB::from_f32(0.0, 1.0, 0.0)),
            TileType::DownStairs => (rltk::to_cp437('>'), RGB::from_f32(0.0, 1.0, 1.0)),
        };
        if !map.visible_tiles[idx] {
            fg = fg.to_greyscale();
        }
        ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
    }
}
