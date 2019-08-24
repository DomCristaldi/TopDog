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
  Components::{
    CharacterMovementStateComponent,
    Dimensions,
  },
};


#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct PaddlePrefab
{
  pub character_movement: CharacterMovementStateComponent,
  pub dimensions: Dimensions,
}

impl PaddlePrefab
{
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<PaddlePrefab>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, PaddlePrefab>|
            {
                loader.load(
                    "paddle.ron",
                    RonFormat,
                    ()
                )
            }
        );

        return prefab_handle;
    }
}