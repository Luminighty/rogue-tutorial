use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(NullStorage)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Potion {
	pub heal_amount: i32
}

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
	pub owner: Entity
}
impl InBackpack {
	pub fn new(owner: Entity) -> Self {
		Self { owner }
	}
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
	pub collected_by: Entity,
	pub item: Entity
}

impl WantsToPickupItem {
	pub fn new(collected_by: Entity, item: Entity) -> Self {
		Self { collected_by, item }
	}
}

#[derive(Component, Debug)]
pub struct WantsToDrinkPotion {
	pub potion: Entity
}

impl WantsToDrinkPotion {
	pub fn new(potion: Entity) -> Self {
		Self { potion }
	}
}

#[derive(Component, Debug)]
pub struct WantsToDropItem {
	pub item: Entity
}
