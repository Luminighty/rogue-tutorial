use rltk::{VirtualKeyCode, RGB};
use specs::{Entity, Join, WorldExt};

use crate::{component::{InBackpack, Name}, resource::player::PlayerEntity, state::State};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ItemMenuResult {
	Cancel,
	NoResponse, 
	Selected(Entity),
}
pub fn show_inventory(state: &mut State, ctx: &mut rltk::Rltk, title: &str) -> ItemMenuResult {
	let player_entity = state.ecs.fetch::<PlayerEntity>().0;
	let names = state.ecs.read_storage::<Name>();
	let backpack = state.ecs.read_storage::<InBackpack>();
	let entities = state.ecs.entities();

	let inventory = (&backpack, &names).join().filter(|item| item.0.owner == player_entity);
	let count = inventory.count();

	let mut y = (25 - (count / 2)) as i32;
	ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
	ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), title);
	ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

	let mut equippable : Vec<Entity> = Vec::new();
	let mut j = 0;
	for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == player_entity ) {
		ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
		ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
		ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

		ctx.print(21, y, &name.name.to_string());
		equippable.push(entity);
		y += 1;
		j += 1;
	}

	match ctx.key {
		None => ItemMenuResult::NoResponse,
		Some(VirtualKeyCode::Escape) => ItemMenuResult::Cancel,
		Some(key) => {
			let selection = rltk::letter_to_option(key);
			if selection > -1 && selection < count as i32 {
				return ItemMenuResult::Selected(equippable[selection as usize]);
			}
			ItemMenuResult::NoResponse
		},
	}
}
