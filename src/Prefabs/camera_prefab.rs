use amethyst::{
  prelude::Builder,
  assets::{
    Handle, Prefab, PrefabData, PrefabLoader, RonFormat, ProgressCounter,
  },
  core::{
    Transform,
  },
  derive::{
    PrefabData,
  },
  ecs::{
    Component, Entity, World, WriteStorage,
  },
  error::Error,
  renderer::camera::{
    //Camera, Projection,
    Camera,
    CameraPrefab,

  },
};

use serde::{
  Serialize,
  Deserialize,
};

use crate::StaticData::StaticData;


#[derive(Debug, Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct CameraPrefabData
{
  pub transform: Transform,
  pub camera: CameraPrefab,
}

impl CameraPrefabData
{
  // TODO: make a macro for this?
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<CameraPrefabData>>
  {
    world.exec( |loader: PrefabLoader<'_, CameraPrefabData>|
    {
      loader.load( "camera.ron", RonFormat, () )
    })
  }

    pub fn initialize(world: &mut World)
  {
    let prefab_handle: Handle<Prefab<CameraPrefabData>> = CameraPrefabData::retrieve_prefab_handle(world);

    let mut transform = Transform::default();
      transform.set_translation_xyz(StaticData::ARENA_WIDTH() * 0.5, StaticData::ARENA_HEIGHT() * 0.5, 1.0);

    world
        .create_entity()
        .with(prefab_handle.clone())
        .build();
  }
}