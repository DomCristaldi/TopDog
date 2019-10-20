use amethyst::{
  error::Error,
  assets::{ PrefabData, },
  derive::{ PrefabData, },
  ecs::{ Component, DenseVecStorage, Entity, WriteStorage, NullStorage},
};

#[derive(Clone, Copy, Debug, Default, PrefabData)]
#[prefab(Component)]
pub struct CharacterTag;

impl Component for CharacterTag
{
  type Storage = NullStorage<Self>;
}