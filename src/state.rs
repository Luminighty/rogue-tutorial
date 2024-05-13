use rltk::console;
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use specs::storage::GenericReadStorage;

use crate::component::*;
use crate::resource::gamelog::GameLog;
use crate::resource::gui;
use crate::resource::gui::show_inventory;
use crate::resource::gui::ItemMenuResult;
use crate::resource::gui::MainMenuResult;
use crate::resource::gui::MainMenuSelection;
use crate::resource::gui::TargetMenuResult;
use crate::resource::insert_resources;
use crate::resource::map::*;
use crate::resource::player::PlayerEntity;
use crate::resource::spawner;
use crate::system;
use crate::system::damage;
use crate::system::player::player_input;
use crate::system::saveload_system;
use crate::templates;


#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
	AwaitingInput,
	PreRun,
	PlayerTurn,
	MonsterTurn,
	ShowInventory,
	ShowDropItem,
	ShowTargeting { range: i32, item: Entity },
	MainMenu { menu_selection: MainMenuSelection },
	SaveGame,
}

impl Default for RunState {
	fn default() -> Self {
		RunState::MainMenu { menu_selection: MainMenuSelection::default() }
	}
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
		self.ecs.insert(RunState::default());
		register_components(&mut self.ecs);
		insert_resources(&mut self.ecs);

		let map = self.ecs.fetch::<Map>();
		let rooms = map.rooms.clone();
		drop(map);
		let (player_x, player_y) = rooms[0].center();
		spawner::spawn_item(&mut self.ecs, &rooms[0], 5);
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
			RunState::MainMenu { menu_selection } => {
				match gui::main_menu(self, ctx, menu_selection) {
					MainMenuResult::NoSelection { selected } => {
						new_run_state = RunState::MainMenu { menu_selection: selected };
					}
					MainMenuResult::Selected { selected } => {
						match selected {
							MainMenuSelection::NewGame => new_run_state = RunState::PreRun,
							MainMenuSelection::LoadGame => {
								saveload_system::load_game(&mut self.ecs);
								new_run_state = RunState::AwaitingInput;
								saveload_system::delete_save();
							},
							MainMenuSelection::Quit => { ctx.quit(); },
						}
					}
				}
			},
			RunState::SaveGame => {
				saveload_system::save_game(&mut self.ecs);
				new_run_state = RunState::MainMenu { menu_selection:  MainMenuSelection::LoadGame }
			}
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
						let is_ranged = self.ecs.read_storage::<Ranged>();
						if let Some(is_ranged) = is_ranged.get(entity) {
							new_run_state = RunState::ShowTargeting { 
								range: is_ranged.range, 
								item: entity 
							}
						} else {
							let mut intent = self.ecs.write_storage::<WantsToUseItem>();
							let player = self.ecs.fetch::<PlayerEntity>().0;
							intent.insert(player, WantsToUseItem::new(entity)).expect("Unable to insert intent");
							new_run_state = RunState::PlayerTurn;
						}

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
			},
			RunState::ShowTargeting { range, item } => {
				match gui::ranged_target(self, ctx, range) {
					TargetMenuResult::NoResponse => {},
					TargetMenuResult::Cancel => { new_run_state = RunState::AwaitingInput; },
					TargetMenuResult::Selected(target) => {
						let mut intent = self.ecs.write_storage::<WantsToUseItem>();
						let player = self.ecs.fetch::<PlayerEntity>().0;
						intent.insert(player, WantsToUseItem::on(item, Some(target)))
							.expect("Failed to target use item");
						new_run_state = RunState::PlayerTurn;
					}
				}
			}
		}

		if damage::DamageSystem::delete_the_dead(&mut self.ecs) {
			self.ecs.delete_all();
			new_run_state = RunState::default();
		}
		{
			let mut run_state = self.ecs.write_resource::<RunState>();
			*run_state = new_run_state;
		}
	}
}

