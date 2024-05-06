use specs::World;

pub mod map;
pub mod player;

pub fn insert_resources(ecs: &mut World) {
	ecs.insert(map::generate_map());
	ecs.insert(player::PlayerData::new(0, 0));
}
