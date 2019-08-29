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
  },
  error::Error,
};

use serde::{
  Serialize,
  Deserialize,
};

use crate::{
  Components::{
    Attributes::{
      CharacterMovementStateComponent,
    },
  }
};


#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct CharacterStatePrefabData
{
  pub ground_movement: CharacterMovementStateComponent,
}