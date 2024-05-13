use rltk::{VirtualKeyCode, RGB};

use crate::{state::State, system::saveload_system};

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum MainMenuSelection {
	#[default] NewGame, 
	LoadGame, 
	Quit
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
	NoSelection { selected: MainMenuSelection },
	Selected { selected: MainMenuSelection },
}


pub fn main_menu(state: &mut State, ctx: &mut rltk::Rltk, selection: MainMenuSelection) -> MainMenuResult {
	let save_exists = saveload_system::has_save_game();
	ctx.print_color_centered(
		15, 
		RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 
		"Rust Roguelike Tutorial"
	);

	menu_item(24, MainMenuSelection::NewGame, selection, "Begin New Game", ctx);
	if save_exists {
		menu_item(25, MainMenuSelection::LoadGame, selection, "Load Game", ctx);
	}
	menu_item(26, MainMenuSelection::Quit, selection, "Quit", ctx);

	match ctx.key {
		Some(VirtualKeyCode::Escape) => MainMenuResult::NoSelection{ selected: MainMenuSelection::Quit },
		Some(VirtualKeyCode::Up) => MainMenuResult::NoSelection { selected: selection_prev(selection, save_exists) },
		Some(VirtualKeyCode::Down) => MainMenuResult::NoSelection { selected: selection_next(selection, save_exists) },
		Some(VirtualKeyCode::Return) => MainMenuResult::Selected{ selected : selection },
		_ => MainMenuResult::NoSelection { selected: selection }
	}
}

fn selection_next(selection: MainMenuSelection, save_exists: bool) -> MainMenuSelection {
	let selection = match selection {
		MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
		MainMenuSelection::LoadGame => MainMenuSelection::Quit,
		MainMenuSelection::Quit => MainMenuSelection::NewGame,
	};
	if !save_exists && selection == MainMenuSelection::LoadGame {
		selection_next(selection, save_exists)
	} else {
		selection
	}
}

fn selection_prev(selection: MainMenuSelection, save_exists: bool) -> MainMenuSelection {
	let selection = match selection {
		MainMenuSelection::NewGame => MainMenuSelection::Quit,
		MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
		MainMenuSelection::Quit => MainMenuSelection::LoadGame,
	};
	if !save_exists && selection == MainMenuSelection::LoadGame {
		selection_prev(selection, save_exists)
	} else {
		selection
	}
}

fn menu_item(y: i32, item: MainMenuSelection, selected: MainMenuSelection, title: &str, ctx: &mut rltk::Rltk) {
	let color = if item == selected { RGB::named(rltk::MAGENTA) } else { RGB::named(rltk::WHITE) };
	ctx.print_color_centered(y, color, RGB::named(rltk::BLACK), title)
}