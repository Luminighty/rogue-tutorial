use rltk::RGB;
use specs::{Builder, World, WorldExt};

use crate::component::*;

pub fn create_player(
	world: &mut World, 
	x: i32, y: i32
) {
	world.create_entity()
	.with(Position::new(x, y))
	.with(Renderable::new(
		rltk::to_cp437('@'),
		RGB::named(rltk::YELLOW),
		RGB::named(rltk::BLACK),
	))
	.with(Viewshed::new(8))
	.with(Player{})
	.build();
}