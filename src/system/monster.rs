use rltk::console;
use specs::{Join, ReadStorage, System};

use crate::component::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
	type SystemData = (
		ReadStorage<'a, Viewshed>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Monster>,
	);

	fn run(&mut self, data: Self::SystemData) {
		let (viewshed, pos, monster) = data;

		for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
			console::log("Monster is very cute :)");
		}
	}
}