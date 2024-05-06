mod utils;
mod component;
mod system;
mod templates;
mod resource;

mod state;


fn main() -> rltk::BError {
	let context = rltk::RltkBuilder::simple80x50()
		.with_title("Roguelike Tutorial")
		.build()?;
	let mut gs = state::State::new();
	gs.setup();
	rltk::main_loop(context, gs)
}
