use rltk::{Point, RGB};
use specs::{Join, World, WorldExt};

use crate::{component::{CombatStats, Name, Player, Position}, resource::{gamelog, map}};

pub fn player_healthbar(ecs: &World, ctx: &mut rltk::Rltk) {
	let combat_stats = ecs.read_storage::<CombatStats>();
	let players = ecs.read_storage::<Player>();
	for (_player, stats) in (&players, &combat_stats).join() {
		let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
		ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

		ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
	}
}

pub fn gamelog(ecs: &World, ctx: &mut rltk::Rltk) {
	let log = ecs.fetch::<gamelog::GameLog>();
	let mut y = 44;
	for entry in log.entries.iter().rev() {
		if y >= 49 { break; }
		ctx.print(2, y, entry);
		y += 1;
	}
}

pub fn draw_tooltips(ecs: &World, ctx: &mut rltk::Rltk) {
	let map = ecs.fetch::<map::Map>();
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
					let padding = (width - s.len() as i32)-1;
					ctx.print_color(left_x + padding - 1, y, FG, BG, s);
					for i in 0..padding {
							ctx.print_color(arrow_pos.x + 1 + i as i32, y, FG, BG, &" ".to_string());
					}
					y += 1;
			}
			ctx.print_color(arrow_pos.x, arrow_pos.y, FG, BG, &"<-".to_string());
	}
}