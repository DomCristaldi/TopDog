use amethyst::{
  prelude::{ World, WorldExt, Builder, },

  assets::{ Handle, Prefab, PrefabData, ProgressCounter, },

  core::{ Transform, },

  ecs::{ /*World,*/ Entity, },

  renderer::{
    debug_drawing::DebugLinesComponent,
  },
};

use crate::{
  Components::{ Dimensions, },
  Prefabs::{ CollidableSurfacePrefab },
};


#[derive(Debug)]
pub struct CollisionSurfaceEntity;

impl CollisionSurfaceEntity
{
  pub fn initialize(world: &mut World)
  {
    let prefab_handle: Handle<Prefab<CollidableSurfacePrefab>>
      = CollidableSurfacePrefab::retieve_prefab_handle(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 50.0, 0.0);

    world
      .create_entity()
      .with(prefab_handle.clone())
      //.with(transform)
      .with(DebugLinesComponent::default())

      .build();
  }
}