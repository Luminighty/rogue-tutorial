use specs_derive::{Component, ConvertSaveload};
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use specs::prelude::*;
use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::saveload::Marker;
use specs::saveload::ConvertSaveload;
use specs::error::NoError;

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

#[derive(Component, Clone, ConvertSaveload)]
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
	ecs.register::<SimpleMarker<SerializeMe>>();
	ecs.register::<SerializationHelper>();
	ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
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
	ecs.register::<ProvidesHealing>();
	ecs.register::<InBackpack>();
	ecs.register::<WantsToPickupItem>();
	ecs.register::<WantsToUseItem>();
	ecs.register::<WantsToDropItem>();
	ecs.register::<Consumable>();
	ecs.register::<Ranged>();
	ecs.register::<InflictsDamage>();
	ecs.register::<AreaOfEffect>();
	ecs.register::<Confusion>();
}
