use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(NullStorage)]
pub struct Player {}
