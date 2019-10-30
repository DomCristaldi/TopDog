use {
  amethyst::{
    assets::{
      Handle,
      Prefab,
      PrefabData,
      PrefabLoader,
      RonFormat,
      ProgressCounter,
    },
    derive::{
      PrefabData,
    },
    ecs::{
      Entity,
      World,
    },
    error::Error,
  },
  serde::{
     Deserialize,
     Serialize,
  },
};

use crate::{
  Compatability::*,
  /*{
    RigBodyDesc_PrefabData,
    ShapeDesc_PrefabData,
  },*/
  Components::{
    Dimensions,
    Attributes::{
      CharacterMovementStateComponent,
      CharacterJumpStateComponent,
      MassAttribute,
    },
  },
};


#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct PaddlePrefab
{
  pub character_movement: CharacterMovementStateComponent,
  pub jump_movement: CharacterJumpStateComponent,
  pub mass: MassAttribute,
  pub dimensions: Dimensions,
  pub physical_desc: Option<PhysicsalDesc>,
  /*pub rigidbody: Option<RigBodyDesc_PrefabData>,
  pub physicalShape: Option<ShapeDesc_PrefabData>,*/
}

impl PaddlePrefab
{
  pub fn prefab_file() -> String
  {
    "paddle.ron".into()
  }

  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<PaddlePrefab>>
  {
    world.exec( |loader: PrefabLoader<'_, PaddlePrefab>|
    {
      loader.load( "paddle.ron", RonFormat, ())
    })
  }
}