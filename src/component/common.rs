use rltk::RGB;
use specs_derive::Component;
use specs::prelude::*;


#[derive(Component)]
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
}

impl Renderable {
	pub fn new(glyph: rltk::FontCharType, fg: RGB, bg: RGB) -> Self {
		Self { glyph, fg, bg, }
	}
}
