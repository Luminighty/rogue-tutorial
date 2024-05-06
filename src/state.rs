use rltk::RandomNumberGenerator;
use specs::prelude::*;

use crate::component::*;
use crate::resource::insert_resources;
use crate::resource::map::*;
use crate::system;
use crate::templates;


#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
	Paused,
	Running
}

pub struct State {
	pub ecs: World,
	pub run_state: RunState
}

impl State {
	pub fn new() -> Self {
		Self {
			ecs: World::new(),
			run_state: RunState::Running
		}
	}


	pub fn setup(&mut self) {
		register_components(&mut self.ecs);
		insert_resources(&mut self.ecs);

		let map = self.ecs.fetch::<Map>();
		let rooms = map.rooms.clone();
		drop(map);
		let (player_x, player_y) = rooms[0].center();
		let mut rng = RandomNumberGenerator::new();
		for room in rooms.iter().skip(1) {
			let (x, y) = room.center();
			templates::create_monster(&mut self.ecs, x, y, &mut rng);
		}

		templates::create_player(&mut self.ecs, player_x, player_y);
	}


	pub fn render(&mut self, ctx: &mut rltk::Rltk) {
		ctx.cls();
		
		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();
		
		for (pos, render) in (&positions, &renderables).join() {
			let idx = xy_idx(pos.x, pos.y);
			if !map.visible_tiles[idx] { continue; }
			ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
		}

		ctx.print(1, 1, "Hello Rogue");
	}
}


impl rltk::GameState for State {
	fn tick(&mut self, ctx: &mut rltk::Rltk) {
		if self.run_state == RunState::Running {
			system::run_systems(self);
			self.run_state = RunState::Paused;
		} else {
			self.run_state = system::player::player_input(self, ctx);
		}
		self.render(ctx);
	}
}

