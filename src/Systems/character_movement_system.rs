extern crate amethyst;

use amethyst::{
  core::{
    Transform,
    timing::Time,
  },
  ecs::{
    Join,
    Read,
    ReadStorage,
    WriteStorage,
    System,
  },
};

use std::time::Duration;

use crate::{
  Components::{
    PaddleComponent,
    InputStatusComponent,
  },
};

pub struct CharacterMovementSystem;

impl<'s> System<'s> for CharacterMovementSystem
{
  type SystemData = (
    WriteStorage<'s, InputStatusComponent>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (mut input_status_components, mut transform_components): Self::SystemData)
  {
    let current_time : f32 = Time::default().fixed_seconds();

    for (input_status, transform) in (&mut input_status_components, &mut transform_components).join()
    {
      transform.move_right(input_status.input_scale * current_time);
    }
  }
}