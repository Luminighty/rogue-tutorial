use rltk::RGB;
use specs::{Builder, World, WorldExt};

use crate::component::{Item, Name, Position, Potion, Renderable};

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
	ecs.create_entity()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph: rltk::to_cp437('¡'),
			fg: RGB::named(rltk::MAGENTA),
			bg: RGB::named(rltk::BLACK),
			render_order: 2,
		})
		.with(Name::new("Health Potion".to_string()))
		.with(Item {})
		.with(Potion { heal_amount: 8 })
		.build();
}
