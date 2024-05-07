use rltk::RGB;
use specs::{Builder, Entity, World, WorldExt};

use crate::component::*;

pub fn create_player(
	world: &mut World, 
	x: i32, y: i32
) -> Entity {
	world.create_entity()
		.with(Position::new(x, y))
		.with(Renderable::new(
			rltk::to_cp437('@'),
			RGB::named(rltk::YELLOW),
			RGB::named(rltk::BLACK),
			0
		))
		.with(Viewshed::new(8))
		.with(Player{})
		.with(Name::new("Player".to_string()))
		.with(CombatStats::new(30, 2, 5))
		.build()
}
