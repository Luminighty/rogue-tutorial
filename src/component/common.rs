use rltk::RGB;
use specs_derive::Component;
use specs::prelude::*;


#[derive(Component, Clone, Copy)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

impl Position {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y, }
	}
}


#[derive(Component)]
pub struct Renderable {
	pub glyph: rltk::FontCharType,
	pub fg: RGB,
	pub bg: RGB,
	pub render_order: i32
}

impl Renderable {
	pub fn new(glyph: rltk::FontCharType, fg: RGB, bg: RGB, render_order: i32) -> Self {
		Self { glyph, fg, bg, render_order }
	}
}

#[derive(Component)]
pub struct Name {
	pub name: String
}

impl Name {
	pub fn unwrap(name: Option<&Name>) -> &str {
		if let Some(name) = name {
			&name.name
		} else {
			"???"
		}
	}
}

impl Name {
	pub fn new<S: ToString>(name: S) -> Self {
		Self { name: name.to_string() }
	}
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct CombatStats {
	pub max_hp: i32,
	pub hp: i32,
	pub defense: i32,
	pub power: i32
}

impl CombatStats {
	pub fn new(max_hp: i32, defense: i32, power: i32) -> Self {
		Self {
			max_hp, hp: max_hp,
			defense, power
		}
	}
}
