use rltk::{Point, VirtualKeyCode, RGB};
use specs::{Entity, Join, World, WorldExt};

use crate::{component::{CombatStats, InBackpack, Name, Player, Position}, state::State};

use super::{gamelog, map::Map, player::PlayerEntity};

pub fn draw_ui(ecs: &World, ctx: &mut rltk::Rltk) {
	ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

	player_healthbar(ecs, ctx);
	gamelog(ecs, ctx);

	let mouse_pos = ctx.mouse_pos();
	ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

	draw_tooltips(ecs, ctx);
}

fn player_healthbar(ecs: &World, ctx: &mut rltk::Rltk) {
	let combat_stats = ecs.read_storage::<CombatStats>();
	let players = ecs.read_storage::<Player>();
	for (_player, stats) in (&players, &combat_stats).join() {
		let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
		ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

		ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
	}
}

fn gamelog(ecs: &World, ctx: &mut rltk::Rltk) {
	let log = ecs.fetch::<gamelog::GameLog>();
	let mut y = 44;
	for entry in log.entries.iter().rev() {
		if y >= 49 { break; }
		ctx.print(2, y, entry);
		y += 1;
	}
}

fn draw_tooltips(ecs: &World, ctx: &mut rltk::Rltk) {
	let map = ecs.fetch::<Map>();
	let names = ecs.read_storage::<Name>();
	let positions = ecs.read_storage::<Position>();

	let mouse_pos = ctx.mouse_pos();
	if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { 
		return; 
	}

	let mut tooltip: Vec<String> = Vec::new();
	for (name, position) in (&names, &positions).join() {
		let idx = map.xy_idx(position.x, position.y);
		if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
			tooltip.push(name.name.to_string());
		}
	}

	if tooltip.is_empty() {
		return;
	}

	let width = tooltip.iter().map(|s| s.len()).max().unwrap_or_default() as i32 + 3;

	const FG: (u8, u8, u8) = rltk::WHITE;
	const BG: (u8, u8, u8) = rltk::GRAY10;

	if mouse_pos.0 > 40 {
		let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
		let left_x = mouse_pos.0 - width;
		let mut y = mouse_pos.1;
		for s in tooltip.iter() {
				ctx.print_color(left_x, y, FG, BG, s);
				let padding = (width - s.len() as i32)-1;
				for i in 0..padding {
						ctx.print_color(arrow_pos.x - i, y, FG, BG, &" ".to_string());
				}
				y += 1;
		}
		ctx.print_color(arrow_pos.x, arrow_pos.y, FG, BG, &"->".to_string());
	} else {
			let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
			let left_x = mouse_pos.0 +3;
			let mut y = mouse_pos.1;
			for s in tooltip.iter() {
					ctx.print_color(left_x + 1, y, FG, BG, s);
					let padding = (width - s.len() as i32)-1;
					for i in 0..padding {
							ctx.print_color(arrow_pos.x + 1 + i, y, FG, BG, &" ".to_string());
					}
					y += 1;
			}
			ctx.print_color(arrow_pos.x, arrow_pos.y, FG, BG, &"<-".to_string());
	}
}

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