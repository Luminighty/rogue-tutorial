use specs_derive::Component;
use specs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Player {}
