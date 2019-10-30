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
  //Compatability::{ RigBodyDesc_PrefabData, },
  Compatability::*,
  Components::{ Dimensions, },
};

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct CollidableSurfacePrefab
{
  pub transform: Transform,
  pub dimensions: Dimensions,
  pub physical_desc: Option<PhysicsalDesc>,
  //pub rigidbody: Option<RigBodyDesc_PrefabData>
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