use rltk::{Point, VirtualKeyCode, RGB};
use specs::{storage::GenericReadStorage, Entity, Join, World, WorldExt};

use crate::{component::{CombatStats, InBackpack, Name, Player, Position, Viewshed}, state::State};

use super::{gamelog, map::Map, player::{PlayerData, PlayerEntity}};

pub mod common;
pub mod inventory;
pub mod targeting;
pub mod mainmenu;

pub use common::*;
pub use inventory::*;
pub use targeting::*;
pub use mainmenu::*;

pub fn draw_ui(ecs: &World, ctx: &mut rltk::Rltk) {
	ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

	player_healthbar(ecs, ctx);
	gamelog(ecs, ctx);

	let mouse_pos = ctx.mouse_pos();
	ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

	draw_tooltips(ecs, ctx);
}


