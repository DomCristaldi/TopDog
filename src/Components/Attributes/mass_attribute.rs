use amethyst::{
  assets::PrefabData,
  derive::PrefabData,
  error::Error,
  ecs::{
    Entity,
    Component,
    VecStorage,
    WriteStorage,
  },
};

use serde::{
  Serialize,
  Deserialize,
};

use derivative::{
  Derivative,
};

/*extern crate derivative;
use derivative::Derivative;*/

#[derive(Clone, Copy, Debug, Derivative, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
#[derivative(Default)]
pub struct MassAttribute
{
  #[derivative(Default(value="1.0"))]
  pub mass: f32,
}

impl Component for MassAttribute
{
  type Storage = VecStorage<Self>;
}