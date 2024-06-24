use rltk::RGB;
use specs::World;

pub mod common;
pub mod inventory;
pub mod mainmenu;
pub mod targeting;

pub use common::*;
pub use inventory::*;
pub use mainmenu::*;
pub use targeting::*;

pub fn draw_ui(ecs: &World, ctx: &mut rltk::Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    player_healthbar(ecs, ctx);
    gamelog(ecs, ctx);

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

    draw_tooltips(ecs, ctx);
    draw_depth(ecs, ctx);
}
