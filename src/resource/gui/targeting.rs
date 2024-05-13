use rltk::{Point, RGB};
use specs::WorldExt;

use crate::{component::Viewshed, resource::player::{PlayerData, PlayerEntity}, state::State};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TargetMenuResult {
	Cancel,
	NoResponse, 
	Selected(Point),
}
pub fn ranged_target(state: &mut State, ctx: &mut rltk::Rltk, range: i32) -> TargetMenuResult {
	let player = state.ecs.fetch::<PlayerEntity>().0;
	let player_data = state.ecs.fetch::<PlayerData>();
	let viewsheds = state.ecs.read_storage::<Viewshed>();

	ctx.print_color(5, 0, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Select Target:");

	let mut available_cells = Vec::new();
	if let Some(visible) = viewsheds.get(player) {
		for idx in visible.visible_tiles.iter() {
			let distance = rltk::DistanceAlg::Pythagoras.distance2d(player_data.position, *idx);
			if distance <= range as f32 {
				ctx.set_bg(idx.x, idx.y, RGB::named(rltk::BLUE));
				available_cells.push(idx);
			}
		}
	} else {
		return TargetMenuResult::Cancel;
	}

	let mouse_pos = ctx.mouse_pos();
	let valid_target = available_cells
		.iter()
		.any(|idx| idx.x == mouse_pos.0 && idx.y == mouse_pos.1);


	

	if valid_target {
		ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));
		if ctx.left_click {
			return TargetMenuResult::Selected(Point::new(mouse_pos.0, mouse_pos.1));
		}
	} else {
		ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
		if ctx.left_click {
			return TargetMenuResult::Cancel;
		}
	}
	TargetMenuResult::NoResponse
}
