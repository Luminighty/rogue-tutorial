use rltk::RandomNumberGenerator;
use specs::prelude::*;

use crate::component::*;
use crate::resource::gamelog::GameLog;
use crate::resource::gui;
use crate::resource::gui::show_inventory;
use crate::resource::gui::ItemMenuResult;
use crate::resource::insert_resources;
use crate::resource::map::*;
use crate::resource::player::PlayerEntity;
use crate::resource::spawner;
use crate::system;
use crate::system::damage;
use crate::system::player::player_input;
use crate::templates;


#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
	AwaitingInput,
	PreRun,
	PlayerTurn,
	MonsterTurn,
	ShowInventory,
	ShowDropItem,
}

pub struct State {
	pub ecs: World,
}

impl State {
	pub fn new() -> Self {
		Self {
			ecs: World::new(),
		}
	}


	pub fn setup(&mut self) {
		self.ecs.insert(RunState::PreRun);
		register_components(&mut self.ecs);
		insert_resources(&mut self.ecs);

		let map = self.ecs.fetch::<Map>();
		let rooms = map.rooms.clone();
		drop(map);
		let (player_x, player_y) = rooms[0].center();
		spawner::spawn_item(&mut self.ecs, &rooms[0]);
		for room in rooms.iter().skip(1) {
			spawner::spawn_room(&mut self.ecs, room);
		}

		let player = templates::create_player(&mut self.ecs, player_x, player_y);
		self.ecs.insert(PlayerEntity(player));
	}


	pub fn render(&mut self, ctx: &mut rltk::Rltk) {
		ctx.cls();
		
		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();

		let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
		data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order) );

		for (pos, render) in data.iter() {
			let idx = map.xy_idx(pos.x, pos.y);
			if !map.visible_tiles[idx] { continue; }
			ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
		}

		ctx.print(1, 1, "Hello Rogue");
	}
}


impl rltk::GameState for State {
	fn tick(&mut self, ctx: &mut rltk::Rltk) {
		let mut new_run_state = {
			*self.ecs.fetch::<RunState>()
		};

		self.render(ctx);
		gui::draw_ui(&self.ecs, ctx);
		match new_run_state {
			RunState::PreRun => {
				system::run_systems(self);
				new_run_state = RunState::AwaitingInput
			},
			RunState::AwaitingInput => {
				new_run_state = player_input(self, ctx);
			},
			RunState::PlayerTurn => {
				system::run_systems(self);
				new_run_state = RunState::MonsterTurn;
			},
			RunState::MonsterTurn => {
				system::run_systems(self);
				new_run_state = RunState::AwaitingInput;
			},
			RunState::ShowInventory => {
				match show_inventory(self, ctx, "Inventory") {
					ItemMenuResult::NoResponse => {},
					ItemMenuResult::Cancel => { new_run_state = RunState::AwaitingInput; },
					ItemMenuResult::Selected(entity) => {
						let mut intent = self.ecs.write_storage::<WantsToDrinkPotion>();
						let player = self.ecs.fetch::<PlayerEntity>().0;
						intent.insert(player, WantsToDrinkPotion::new(entity)).expect("Unable to insert intent");
						new_run_state = RunState::PlayerTurn;
					}
				}
			},
			RunState::ShowDropItem => {
				match show_inventory(self, ctx, "Drop which item?") {
					ItemMenuResult::NoResponse => {},
					ItemMenuResult::Cancel => { new_run_state = RunState::AwaitingInput; },
					ItemMenuResult::Selected(item) => {
						let mut intent = self.ecs.write_storage::<WantsToDropItem>();
						let player = self.ecs.fetch::<PlayerEntity>().0;
						intent.insert(player, WantsToDropItem { item }).expect("Unable to insert intent");
						new_run_state = RunState::PlayerTurn;
					}
				}
			}
		}

		{
			let mut run_state = self.ecs.write_resource::<RunState>();
			*run_state = new_run_state;
		}
		damage::DamageSystem::delete_the_dead(&mut self.ecs);
	}
}

