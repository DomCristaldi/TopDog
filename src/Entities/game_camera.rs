extern crate amethyst;

use amethyst::{
    prelude::*,
    error::Error,
    assets::{
        AssetStorage,
        Handle,
        Prefab,
        PrefabData,
        PrefabLoader,
        PrefabLoaderSystem,
        ProgressCounter,
        RonFormat
        },
    core::transform::Transform,
    derive::PrefabData,
    ecs::Entity,
    ecs::prelude::{
        Component,
        DenseVecStorage,
        WriteStorage 
        },
    renderer::{
      Camera,
      ImageFormat,
      SpriteRender,
      SpriteSheet,
      SpriteSheetFormat,
      Texture
      },
    utils::scene::BasicScenePrefab,
};

use serde::{Deserialize, Serialize};
use specs_derive::Component;

use crate::StaticData::StaticData;
use crate::Resources;

#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct GameCamera
{
  pub position: Resources::Position,
  pub dimensions: Resources::Dimensions,
}

/*impl PrefabData<'a> for GameCamera
{
  type SystemData = WriteStorage<'a, Dimensions>;

  fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity],
    ) -> Result<Self::Result, Error>
    {

    }
}*/

impl GameCamera
{
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<GameCamera>>
  {
    let prefab_handle = world.exec(
      |loader: PrefabLoader<'_, GameCamera>|
      {
        loader.load(
          "game_camera.ron",
          RonFormat,
          ()
        )
      }
    );

    return prefab_handle;
  }

  pub fn initialize(world: &mut World)
  {
    let prefab_handle: Handle<Prefab<GameCamera>> = GameCamera::retrieve_prefab_handle(world);

    let mut transform = Transform::default();
      transform.set_translation_xyz(StaticData::ARENA_WIDTH() * 0.5, StaticData::ARENA_HEIGHT() * 0.5, 1.0);

    world
        .create_entity()
        .with(prefab_handle.clone())
        .with(Camera::standard_2d(StaticData::ARENA_WIDTH(), StaticData::ARENA_HEIGHT()))
        .with(transform)
        .build();
/*
    world.create_entity()
      .with(prefab_handle.clone())
      .build();*/
  }
}