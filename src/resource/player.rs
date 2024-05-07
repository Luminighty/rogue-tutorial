use rltk::Point;
use specs::Entity;

pub struct PlayerData {
	pub position: Point
}

impl PlayerData {
	pub fn new(x: i32, y: i32) -> Self {
		Self { position: Point::new(x, y) }
	}
}


pub struct PlayerEntity(pub Entity);
