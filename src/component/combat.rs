use specs_derive::Component;
use specs::prelude::*;


#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
	pub target: Entity
}

#[derive(Component)]
pub struct SufferDamage {
	pub amount: Vec<i32>
}

impl SufferDamage {
	pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
		if let Some(suffering) = store.get_mut(victim) {
			suffering.amount.push(amount);
		} else {
			let dmg = SufferDamage { amount: vec![amount] };
			store.insert(victim, dmg).expect("Unable to insert damage");
		}
	}
}
