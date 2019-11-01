use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
};

use crate::{
    Entities::{
        PaddleEntity,
        //GameCamera,
        CollisionSurfaceEntity,
    },
    Prefabs::
    {
        CameraPrefabData,
    },
    StaticData::StaticData,
};

/*fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers the whole arena and (0, 0) is in the bootom left
    let mut transform = Transform::default();
    transform.set_translation_xyz(StaticData::ARENA_WIDTH() * 0.5, StaticData::ARENA_HEIGHT() * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(StaticData::ARENA_WIDTH(), StaticData::ARENA_HEIGHT()))
        .with(transform)
        .build();
}*/


pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;
        CameraPrefabData::initialize(world);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 50.0, 0.0);
        PaddleEntity::initialize(world, Some(transform));

        CollisionSurfaceEntity::initialize(world);
    }
}