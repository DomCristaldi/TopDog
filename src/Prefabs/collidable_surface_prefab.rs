use amethyst::{
  ecs::{ World, },
  assets::{ Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat, },
  core::{ Transform, },
  derive::{ PrefabData, },
  ecs::{ Entity, },
  error::Error,
};

use serde::{
  Serialize, Deserialize,
};

use crate::{
  Components::{ Dimensions, },
};

#[derive(Debug, Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct CollidableSurfacePrefab
{
  pub transform: Transform,
  pub dimensions: Dimensions,
}

impl CollidableSurfacePrefab
{
  pub fn retieve_prefab_handle(world: &mut World) -> Handle<Prefab<CollidableSurfacePrefab>>
  {
    world.exec( |loader: PrefabLoader<'_, CollidableSurfacePrefab>|
    {
      loader.load( "collidable_surface.ron", RonFormat, ())
    })
  }
}