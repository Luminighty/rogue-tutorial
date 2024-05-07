use specs::World;

pub mod map;
pub mod player;
pub mod gui;
pub mod gamelog;
pub mod spawner;

pub fn insert_resources(ecs: &mut World) {
	ecs.insert(map::generate_map());
	ecs.insert(player::PlayerData::new(0, 0));
	ecs.insert(gamelog::GameLog::new());
	ecs.insert(rltk::RandomNumberGenerator::new());
}
