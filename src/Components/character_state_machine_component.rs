extern crate strum;
extern crate strum_macros;

use amethyst::{
  prelude::*,
  ecs::prelude::{
    Component,
    DenseVecStorage,
    WriteStorage 
  },  
};

pub struct CharacterMovementAtributes
{
  pub accel: f32
}

pub enum CharacterStackState
{
  Solo,
  TopDog,
  StackDog,
}

pub enum CharacterMobilityState
{
  Grounded,
  Jump,
  Stomp,
}

pub struct CharacterState
{
  mobility_state: CharacterMobilityState,
  stack_state: CharacterStackState,

  //stack_height: u8,
}

impl Component for CharacterState
{
  type Storage = DenseVecStorage<Self>;
}

impl CharacterState
{
  pub fn CanTransitionToState(&self, next_state: CharacterState) -> bool
  {
    match self.mobility_state
    {
      CharacterMobilityState::Grounded =>
      {
        match next_state
        {
          _ => false,
        }
      },

      CharacterMobilityState::Jump =>
      {
        /*valid_state_transitions!(
          target: next_state,
          [CharacterState::Jump, CharacterState::Stomp]);*/

        match next_state
        {
          _ => false,
        }
      },


      CharacterMobilityState::Stomp =>
      {
        match next_state
        {
          _ => false,
        }
      },
    }
  }
}
