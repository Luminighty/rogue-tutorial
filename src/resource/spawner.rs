use rltk::RandomNumberGenerator;
use specs::{World, WorldExt};

use crate::{templates, utils::rect::Rect};

use super::random_table::RandomTable;

pub const MAX_SPAWNS: i32 = 4;

pub fn spawn_room(ecs: &mut World, room: &Rect, depth: i32) {
    let spawn_table = room_table(depth);
    let mut spawns = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let amount = rng.roll_dice(1, MAX_SPAWNS) + depth;
        for point in select_points(amount, &mut rng, room).iter() {
            spawns.push((*point, spawn_table.roll(&mut rng)));
        }
    };

    for (point, spawner) in spawns {
        let x = point.0 as i32;
        let y = point.1 as i32;

        if let Some(entry) = spawner {
            (entry.spawner)(ecs, x, y);
        }
    }
}

fn select_points(
    amount: i32,
    rng: &mut rltk::RandomNumberGenerator,
    room: &Rect,
) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    for _ in 0..amount {
        let mut added = false;
        while !added {
            let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
            if !points.contains(&(x, y)) {
                points.push((x, y));
                added = true;
            }
        }
    }

    points
}

fn room_table(depth: i32) -> RandomTable {
    use templates::*;
    RandomTable::new()
        .add(goblin, 10 + depth)
        .add(orc, 1 + depth)
        .add(health_potion, 7)
        .add(fireball_scroll, 2)
        .add(confusion_scroll, 2)
        .add(magic_missile_scroll, 4)
}
