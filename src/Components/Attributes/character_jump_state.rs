use amethyst::{
  assets::PrefabData,
  derive::PrefabData,
  error::Error,
  ecs::Entity,
  ecs::prelude::{
    Component,
    DenseVecStorage,
    WriteStorage,
  }
};

use serde::{ Serialize, Deserialize };


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CharacterJumpStateComponent
{
  pub max_jump_height:f32,
  pub time_to_max_height_sec: f32,
}

impl Component for CharacterJumpStateComponent
{
  type Storage = DenseVecStorage<Self>;
}