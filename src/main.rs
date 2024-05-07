mod utils;
mod component;
mod system;
mod templates;
mod resource;

mod state;


fn main() -> rltk::BError {
	let mut context = rltk::RltkBuilder::simple80x50()
		.with_title("Roguelike Tutorial")
		.with_fullscreen(true)
		.build()?;
	context.with_post_scanlines(true);
	context.with_mouse_visibility(false);

	let mut gs = state::State::new();
	gs.setup();
	rltk::main_loop(context, gs)
}
