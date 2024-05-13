use std::{fs::{self, File}, path::Path};
use specs::{saveload::{MarkedBuilder, SimpleMarker, SimpleMarkerAllocator}, Builder, Entity, Join, World, WorldExt};
use crate::{component::*, resource::{map::{self, Map}, player::{PlayerData, PlayerEntity}}};
#[allow(deprecated)]
use specs::error::NoError;
use specs::saveload::{SerializeComponents, DeserializeComponents};

const SAVE_FILE_NAME: &'static str = "./savegame.json";

macro_rules! serialize_individually {
	($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
		$(
			#[allow(deprecated)]
			SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
					&( $ecs.read_storage::<$type>(), ),
					&$data.0,
					&$data.1,
					&mut $ser,
			)
			.unwrap();
			)*
	};
}

macro_rules! deserialize_individually {
	($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
			$(
				#[allow(deprecated)]
			DeserializeComponents::<NoError, _>::deserialize(
					&mut ( &mut $ecs.write_storage::<$type>(), ),
					&mut $data.0, // entities
					&mut $data.1, // marker
					&mut $data.2, // allocater
					&mut $de,
			)
			.unwrap();
			)*
	};
}

pub fn save_game(ecs: &mut World) {
	let mapcopy = ecs.get_mut::<map::Map>().unwrap().clone();
	let savehelper = ecs
		.create_entity()
		.with(SerializationHelper { map: mapcopy })
		.marked::<SimpleMarker<SerializeMe>>()
		.build();

	{
		let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>() );

		let writer = File::create(SAVE_FILE_NAME).unwrap();
		let mut serializer = serde_json::Serializer::new(writer);
		serialize_individually!(ecs, serializer, data, 
			Position, Renderable, Player, Viewshed, Monster, 
			Name, BlocksTile, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, 
			AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem,
			WantsToDropItem, SerializationHelper);
	}

	ecs.delete_entity(savehelper).expect("Failed to cleanup during save");
}

pub fn has_save_game() -> bool {
	Path::new(SAVE_FILE_NAME).exists()
}

pub fn load_game(ecs: &mut World) {
	ecs.delete_all();

	let data = fs::read_to_string(SAVE_FILE_NAME).unwrap();
	let mut de = serde_json::Deserializer::from_str(&data);
	{
		let mut d = (
			&mut ecs.entities(), 
			&mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), 
			&mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>()
		);
		deserialize_individually!(ecs, de, d, Position, Renderable, Player, Viewshed, Monster, 
			Name, BlocksTile, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, 
			AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem,
			WantsToDropItem, SerializationHelper
		);
	}

	let mut deleteme: Option<Entity> = None;
	{
		let entities = ecs.entities();
		let helper = ecs.read_storage::<SerializationHelper>();
		let player = ecs.read_storage::<Player>();
		let position = ecs.read_storage::<Position>();
		for (e, h) in (&entities, &helper).join() {
			let mut worldmap = ecs.write_resource::<Map>();
			*worldmap = h.map.clone();
			worldmap.tile_content = vec![Vec::new(); map::SIZE];
			deleteme = Some(e);
		}
		for (e, _p, pos) in (&entities, &player, &position).join() {
			let mut ppos = ecs.write_resource::<PlayerData>();
			*ppos = PlayerData::new(pos.x, pos.y);
			let mut player_resource = ecs.write_resource::<PlayerEntity>();
			*player_resource = PlayerEntity(e);
		}
	}
	ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper!");
}

pub fn delete_save() {
	if Path::new(SAVE_FILE_NAME).exists() {
		std::fs::remove_file(SAVE_FILE_NAME).expect("Unable to delete file");
	}
}