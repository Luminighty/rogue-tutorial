use rltk::{field_of_view, Point};
use specs::*;
use crate::component::*;
use crate::resource::map::Map;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
	type SystemData = (
		WriteExpect<'a, Map>,
		Entities<'a>,
		WriteStorage<'a, Viewshed>, 
		WriteStorage<'a, Position>,
		ReadStorage<'a, Player>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (
			mut map,
			entities,
			mut viewshed,
			pos,
			player,
		) = data;

		for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
			if !viewshed.dirty { continue; }
			viewshed.dirty = false;
			viewshed.visible_tiles.clear();
			viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
			viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

			if let Some(_player) = player.get(ent) {
				for t in map.visible_tiles.iter_mut() {
					*t = false;
				}
				for tile in viewshed.visible_tiles.iter() {
					let idx = map.xy_idx(tile.x, tile.y);
					map.revealed_tiles[idx] = true;
					map.visible_tiles[idx] = true;
				}
			}
		}
	}
}
