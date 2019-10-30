use amethyst::{
  core::Transform,
};

use crate::Resources::PrefabType;

pub struct LevelPrefab
{
  pub transform: Transform,
  pub prefab: PrefabType,
}