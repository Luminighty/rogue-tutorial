use rltk::{RandomNumberGenerator, RGB};
use specs::{
    saveload::{MarkedBuilder, SimpleMarker},
    Builder, World, WorldExt,
};

use crate::component::*;

use super::render_order;

pub fn create_monster(world: &mut World, x: i32, y: i32) {
    let roll = {
        world
            .write_resource::<RandomNumberGenerator>()
            .roll_dice(1, 2)
    };
    match roll {
        1 => orc(world, x, y),
        _ => goblin(world, x, y),
    }
}

pub fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

pub fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, name: S) {
    ecs.create_entity()
        .marked::<SimpleMarker<SerializeMe>>()
        .with(Position::new(x, y))
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: render_order::MONSTER,
        })
        .with(Viewshed::new(8))
        .with(Monster::new())
        .with(Name::new(name.to_string()))
        .with(BlocksTile {})
        .with(CombatStats::new(16, 1, 4))
        .build();
}
