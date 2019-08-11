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
    ReadStorage<'s, PaddleComponent>,
    WriteStorage<'s, InputStatusComponent>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (paddle_components, mut input_status_components, mut transform_components): Self::SystemData)
  {
    let current_time : f32 = Time::default().fixed_seconds();

    println!("current time: {}", current_time);

    let move_speed = current_time * 300.0;

    for (paddle, input_status, transform) in (&paddle_components, &mut input_status_components, &mut transform_components).join()
    {
      //transform.move_right(input_status.input_scale * current_time);
      transform.move_right(input_status.input_scale * move_speed);

      println!("Movement: {:#?}", input_status);
    }
  }
}