use amethyst::{
  ecs::{
    System,
    Entities,
    Join,
    Read,
    ReadStorage,
    WriteStorage,
  },
};

use crate::{
  Components::{
    InputStatusComponent,
    Velocity2D,
    Movement::{
      CharacterMovementStateComponent,
    },
    Status::{
      JumpStatusComponent,
    },
  },
};


pub struct CharacterJumpSystem;

impl<'s> System<'s> for CharacterJumpSystem
{
  type SystemData = (
    ReadStorage<'s, JumpStatusComponent>,
    ReadStorage<'s, InputStatusComponent>,
    ReadStorage<'s, CharacterMovementStateComponent>,
    WriteStorage<'s, Velocity2D>,
  );

  fn run(&mut self, (jump_status_comps, input_status_comps, movement_state_comps, mut velocity_comps): Self::SystemData)
  {
    for (jump_status, input_status, movement_state, mut velocity)
      in (&jump_status_comps, &input_status_comps, &movement_state_comps, &mut velocity_comps).join()
    {

    }
  }
}