use specs::{Entities, Join, ReadStorage, System, WriteExpect};

use crate::{component::{BlocksTile, Position}, resource::map::Map};

pub struct MapIndexingSystem;

impl<'a> System<'a> for MapIndexingSystem {
	type SystemData = (
		WriteExpect<'a, Map>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, BlocksTile>,
		Entities<'a>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (mut map, position, blockers, entities) = data;
		map.populate_blocked();
		map.clear_content_index();
		for (entity, position) in (&entities, &position).join() {
			let idx = map.xy_idx(position.x, position.y);
			if let Some(_) = blockers.get(entity) {
				map.blocked[idx] = true;
			}
			map.tile_content[idx].push(entity);
		}
	}
}