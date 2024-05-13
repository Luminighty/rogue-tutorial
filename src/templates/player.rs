use rltk::RGB;
use specs::{saveload::{MarkedBuilder, SimpleMarker}, Builder, Entity, World, WorldExt};

use crate::component::*;

use super::render_order;

pub fn create_player(
	world: &mut World, 
	x: i32, y: i32
) -> Entity {
	world.create_entity()
		.marked::<SimpleMarker<SerializeMe>>()
		.with(Position::new(x, y))
		.with(Renderable::new(
			rltk::to_cp437('@'),
			RGB::named(rltk::YELLOW),
			RGB::named(rltk::BLACK),
			render_order::PLAYER
		))
		.with(Viewshed::new(8))
		.with(Player{})
		.with(Name::new("Player".to_string()))
		.with(CombatStats::new(30, 2, 5))
		.build()
}
