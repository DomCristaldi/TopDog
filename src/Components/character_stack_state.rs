use amethyst::{
  ecs::prelude::{
    Component,
    DenseVecStorage,
    WriteStorage,
  }
};

pub enum CharacterStackState
{
  Solo,
  StackMember{level: u8},
}

pub struct CharacterStackStateComponent
{
  pub stack_state: CharacterStackState,
}

impl Component for CharacterStackStateComponent
{
  type Storage = DenseVecStorage<Self>;
}

impl Default for CharacterStackStateComponent
{
  fn default() -> CharacterStackStateComponent
  {
    CharacterStackStateComponent
    {
      stack_state: CharacterStackState::Solo,
    }
  }
}