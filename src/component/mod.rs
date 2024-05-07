use specs_derive::Component;
use specs::prelude::*;

mod item;
mod common;
mod enemy;
mod player;
mod combat;

pub use item::*;
pub use common::*;
pub use enemy::*;
pub use player::*;
pub use combat::*;

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
	ecs.register::<Name>();
	ecs.register::<BlocksTile>();
	ecs.register::<CombatStats>();
	ecs.register::<WantsToMelee>();
	ecs.register::<SufferDamage>();
	ecs.register::<Item>();
	ecs.register::<Potion>();
	ecs.register::<InBackpack>();
	ecs.register::<WantsToPickupItem>();
	ecs.register::<WantsToDrinkPotion>();
	ecs.register::<WantsToDropItem>();
}
