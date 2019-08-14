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

use strum_macros::{
  EnumCount,
  EnumIter,
};


#[derive(Copy, Clone, EnumCount, EnumIter)]
pub enum CharacterStateType
{
  Grounded,
  Jump,
  Stomp,
  TopDog,
  StackDog,
}

impl CharacterStateType
{
  pub fn CanTransitionFrom(source_state: CharacterStateType, next_state: CharacterStateType) -> bool
  {
    match source_state
    {
      CharacterStateType::Grounded =>
      {
        let check_list = [CharacterStateType::Jump, CharacterStateType::Stomp, CharacterStateType::TopDog, CharacterStateType::StackDog,];
        return check_list.contains(CharacterStateType::Grounded);
        for e in 
        {

        }*

        // TOOD: This method works. Look into writing Proc Macro for this, it's gonna explode
        match next_state
        {
          CharacterStateType::Jump => true,
          CharacterStateType::Stomp => false,
          CharacterStateType::TopDog => true,
          CharacterStateType::StackDog => true,
          _ => false
        }
      },
      _ => false
    }
  }

  /*pub fn GetValidTransitionIntoStates(current_state: CharacterStateType) -> Option<[CharacterStateType; 5]>
  {
    match current_state
    {

      _ => None(),
    }
  }*/
}

pub enum MoveDirec
{
  Left, Right,
}

pub enum CharacterStateMachineEvent
{
  JumpEvent,
  StompEvent,
  MoveEvent(MoveDirec),
}

pub enum StateTrans
{
  ChangeState(CharacterStateType),
  Fail,
}

/* --------------------------------------- */

pub trait CharacterStateTrait
{
  fn GetStateType() -> CharacterStateType;

  //fn CheckCanTransition(target_next_state: CharacterStateTrait) -> bool;

  fn Run() -> StateTrans;

  /*fn Execute(&mut target_state: CharacterStaet)
  {
    
  }*/
}

/* --------------------------------------- */

pub struct CharacterStateMachine
{
  //current_state: CharacterStateTrait,
}

impl CharacterStateMachine
{
  //fn Evaluate(&mut self, )
}

/* --------------------------------------- */



/*pub struct GroundedState;

impl CharacterStateData for GroundedState
{
  fn CheckCanTransition(target_next_state: CharacterState) -> bool { true }
}*/

//impl CharacterStateData



pub struct CharacterStateMachineComponent
{
  //pub current_state: CharacterState
  pub state_machine: CharacterStateMachine,
}

impl Component for CharacterStateMachineComponent
{
  type Storage = DenseVecStorage<Self>;
}

/*impl CharacterStateMachine
{
  pub fn GetNextState(character_state: CharacterState) -> StateTrans
  {
    match character_state
    {
      Grounded =>
      {
        return StateTrans::ChangeState(CharacterState::Jump);
      },
      Jump =>
      {

      },

    }

    return StateTrans::Fail;
  }
}*/

