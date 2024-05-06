use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug)]
pub struct Monster {}

impl Monster {
	pub fn new() -> Self {
		Self { }
	}
}
