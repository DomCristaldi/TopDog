use amethyst::{
  assets::{
    PrefabData,
    ProgressCounter,
  },
  derive::{
    PrefabData,
  },
  ecs::{
    Entity,
    WriteStorage,
  },
  error::Error,
};

use serde::{
  Serialize,
  Deserialize,
};


//#[prefab(Component)]

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterMovementAttributes
{
  //pub context: MovementContext,
  pub accel: f32,
  pub decel: f32,
  pub max_speed: f32,
  //pub move_direction_modifier: math::Vector2<f32>,
}

/*impl<'a> PrefabData<'a> for CharacterMovementAttributes
{
  type SystemData = WriteStorage<'a, 
}*/