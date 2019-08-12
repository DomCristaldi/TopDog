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
    //Entities<'s>,
    //ReadStorage<'s, Dimensions>,
    //ReadStorage<'s, Handle<Prefab<PaddleComponent>>>,
    ReadStorage<'s, Dimensions>,
    WriteStorage<'s, InputStatusComponent>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (dimensions_component, mut input_status_components, mut transform_components): Self::SystemData)
  {
    let current_time : f32 = Time::default().fixed_seconds();

    //println!("current time: {}", current_time);

    //let move_speed = current_time * 300.0;

    for (dimensions, input_status, transform) in (dimensions_component.maybe(), &mut input_status_components, &mut transform_components).join()
    {
      //let prefab_assets = prefab_handle.

      match dimensions
      {
        Some(data) =>
        {
          let Dimensions(x, y) = data;
          
          transform.move_right(input_status.input_scale * current_time * x);
        },
        None => (),
      }


      //transform.move_right(input_status_scale * current_time * )
      //println!("Movement: {:#?}", input_status);
    }

    /*for (entity, input_status, transform) in (&*entities, &mut input_status_components, &mut transform_components).join()
    {
      //transform.move_right(input_status.input_scale * current_time);
      transform.move_right(input_status.input_scale * move_speed);

      println!("Movement: {:#?}", input_status);
    }*/
  }
}