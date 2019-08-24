use amethyst::{
  assets::PrefabData,
  derive::PrefabData,
  error::Error,
  /*core::{
    math,
  },*/
  ecs::Entity,
  ecs::prelude::{
    Component,
    DenseVecStorage,
    WriteStorage,
  }
};

use serde::{Deserialize, Serialize};
use specs_derive::Component;


pub enum MovementContext
{
  Grounded,
  Air,
  Stomp,
}

//#[derive(Debug, Clone, Debug, Serialize, Deserialize)]

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct CharacterMovementStateComponent
{
  //pub context: MovementContext,
  pub accel: f32,
  pub decel: f32,
  pub max_speed: f32,
  //pub move_direction_modifier: math::Vector2<f32>,
}

impl Component for CharacterMovementStateComponent
{
  type Storage = DenseVecStorage<Self>;
}

impl Default for CharacterMovementStateComponent
{
  fn default() -> CharacterMovementStateComponent
  {
    CharacterMovementStateComponent
    {
      //context: MovementContext::Grounded,
      accel: 5.0,
      decel: 5.0,
      max_speed: 20.0,
      //move_direction_modifier: math::Vector2::new(1.0, 1.0),
    }
  }
}