use rltk::Point;
use specs_derive::{Component, ConvertSaveload};
use specs::prelude::*;
use serde::{Deserialize, Serialize};
use specs::saveload::*;
use specs::error::NoError;

#[derive(Component, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Item {}

#[derive(Component, Clone, ConvertSaveload)]
pub struct ProvidesHealing {
	pub heal_amount: i32
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct InBackpack {
	pub owner: Entity
}
impl InBackpack {
	pub fn new(owner: Entity) -> Self {
		Self { owner }
	}
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct WantsToPickupItem {
	pub collected_by: Entity,
	pub item: Entity
}

impl WantsToPickupItem {
	pub fn new(collected_by: Entity, item: Entity) -> Self {
		Self { collected_by, item }
	}
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct WantsToUseItem {
	pub item: Entity,
	pub target: Option<rltk::Point>
}

impl WantsToUseItem {
	pub fn new(item: Entity) -> Self {
		Self { item, target: None }
	}
	
	pub fn on(item: Entity, target: Option<Point>) -> Self {
		Self { item, target }
	}
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct WantsToDropItem {
	pub item: Entity
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Consumable {}

#[derive(Component, Clone, ConvertSaveload)]
pub struct Ranged {
	pub range: i32
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct InflictsDamage {
	pub damage: i32,
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct AreaOfEffect {
	pub radius: i32,
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct Confusion {
	pub turns: i32
}
