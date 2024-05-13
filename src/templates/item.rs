use rltk::{RandomNumberGenerator, RGB};
use specs::{saveload::{MarkedBuilder, SimpleMarker}, Builder, World, WorldExt};

use crate::component::*;

use super::render_order;

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
	ecs.create_entity()
		.marked::<SimpleMarker<SerializeMe>>()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph: rltk::to_cp437('¡'),
			fg: RGB::named(rltk::MAGENTA),
			bg: RGB::named(rltk::BLACK),
			render_order: render_order::ITEM,
		})
		.with(Name::new("Health Potion".to_string()))
		.with(Item {})
		.with(ProvidesHealing { heal_amount: 8 })
		.with(Consumable {})
		.build();
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
	ecs.create_entity()
		.marked::<SimpleMarker<SerializeMe>>()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph: rltk::to_cp437(')'),
			fg: RGB::named(rltk::CYAN),
			bg: RGB::named(rltk::BLACK),
			render_order: render_order::ITEM,
		})
		.with(Name::new("Magic Missile Scroll".to_string()))
		.with(Item {})
		.with(Consumable {})
		.with(Ranged { range: 6 })
		.with(InflictsDamage { damage: 8 })
		.build();
}

pub fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
	ecs.create_entity()
		.marked::<SimpleMarker<SerializeMe>>()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph: rltk::to_cp437(')'),
			fg: RGB::named(rltk::ORANGE),
			bg: RGB::named(rltk::BLACK),
			render_order: render_order::ITEM,
		})
		.with(Name::new("Fireball Scroll".to_string()))
		.with(Item {})
		.with(Consumable {})
		.with(Ranged { range: 6 })
		.with(InflictsDamage{ damage: 20 })
		.with(AreaOfEffect { radius: 3 })
		.build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
	ecs.create_entity()
		.marked::<SimpleMarker<SerializeMe>>()
		.with(Position::new(x, y))
		.with(Renderable {
			glyph: rltk::to_cp437(')'),
			fg: RGB::named(rltk::PINK),
			bg: RGB::named(rltk::BLACK),
			render_order: render_order::ITEM,
		})
		.with(Name::new("Confusion Scroll".to_string()))
		.with(Item {})
		.with(Consumable {})
		.with(Ranged { range: 6 })
		.with(Confusion { turns: 4 })
		.build();
}

pub fn random_item(ecs: &mut World, x: i32, y: i32) {
	let roll = { 
		ecs.write_resource::<RandomNumberGenerator>().roll_dice(1, 4)
	};
	match roll {
		1 => magic_missile_scroll(ecs, x, y),
		2 => fireball_scroll(ecs, x, y),
		3 => confusion_scroll(ecs, x, y),
		_ => health_potion(ecs, x, y),
	}
}
