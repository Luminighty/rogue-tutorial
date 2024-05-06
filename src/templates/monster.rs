use rltk::RGB;
use specs::{Builder, World, WorldExt};

use crate::component::*;

pub fn create_monster(
	world: &mut World, 
	x: i32, y: i32,
	rng: &mut rltk::RandomNumberGenerator
) {
	let glyph = match rng.roll_dice(1, 2) {
		1 => rltk::to_cp437('g'),
		_ => rltk::to_cp437('o'),
	};

	world.create_entity()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph,
			fg: RGB::named(rltk::RED),
			bg: RGB::named(rltk::BLACK),
		})
		.with(Viewshed::new(8))
		.with(Monster::new())
		.build();
}