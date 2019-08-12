extern crate amethyst;

use amethyst::{
  assets::{
    Handle,
    Prefab,
  },
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
    Entities,
  },
};

use std::time::Duration;

use crate::{
  Components::{
    Dimensions,
    InputStatusComponent,
  },
};

pub struct CharacterMovementSystem;

impl<'s> System<'s> for CharacterMovementSystem
{
  type SystemData = (
    ReadStorage<'s, Dimensions>,
    WriteStorage<'s, InputStatusComponent>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (dimensions_component, mut input_status_components, mut transform_components): Self::SystemData)
  {
    let current_time : f32 = Time::default().fixed_seconds();

    for (dimensions, input_status, transform) in (dimensions_component.maybe(), &mut input_status_components, &mut transform_components).join()
    {
      match dimensions
      {
        Some(data) =>
        {
          let Dimensions(x, y) = data;
          
          transform.move_right(input_status.input_scale * current_time * x);
        },
        None => (),
      }
    }
  }
}