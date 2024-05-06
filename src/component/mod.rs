use specs_derive::Component;
use specs::prelude::*;

mod common;
mod enemy;
mod player;

pub use common::*;
pub use enemy::*;
pub use player::*;

#[derive(Component, Default)]
pub struct Viewshed {
	pub visible_tiles: Vec<rltk::Point>,
	pub range: i32,
	pub dirty: bool,
}

impl Viewshed {
	pub fn new(range: i32) -> Self {
		Self { visible_tiles: Vec::new(), range, dirty: true }
	} 
}


pub fn register_components(ecs: &mut World) {
	ecs.register::<Position>();
	ecs.register::<Renderable>();
	ecs.register::<Player>();
	ecs.register::<Viewshed>();
	ecs.register::<Monster>();
}
