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

use std::mem::discriminant;

use crate::{
  Components::{
    //Dimensions,
    InputStatusComponent,
    /*CharacterAttributes,
    CharacterAttributeData,
    AttributeCategory,*/
    CharacterMovementStateComponent,
    Velocity2D,
  },
};

pub struct CharacterMovementSystem;

impl<'s> System<'s> for CharacterMovementSystem
{
  type SystemData = (
    //ReadStorage<'s, Dimensions>,
    //ReadStorage<'s, CharacterAttributes>,
    ReadStorage<'s, CharacterMovementStateComponent>,
    WriteStorage<'s, InputStatusComponent>,
    WriteStorage<'s, Velocity2D>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (character_movement_state_comp, mut input_status_components, mut velocity_components, mut transform_components): Self::SystemData)
  {
    let current_time : f32 = Time::default().fixed_seconds();

    for (movement_state, input_status, velocity, transform) in (character_movement_state_comp.maybe(), &mut input_status_components, &mut velocity_components, &mut transform_components).join()
    {
      match movement_state
      {
        Some(data) =>
        {
          let input_contrib:f32 = input_status.input_scale;
          
          let current_frame_accel = match input_contrib > 0.0
          {
            true => 
            {
              input_contrib * data.accel
            },
            false =>
            {
              input_contrib * data.decel * -1.0
            },
          };

          //let target_vel: Velocity2D = Velocity2D::new(velocity.vel.x * input_contrib, velocity.vel.y);
          let mut target_vel = Velocity2D::default();
          target_vel.vel.x = velocity.vel.x * input_contrib;
          target_vel.vel.y = velocity.vel.y;

          //let target_vel_this_frame


          velocity.MoveTowards(&target_vel, &data.max_speed);

          /*match std::mem::discriminant(data.attributes)
          {
            AttributeCategory::Ground(..) =>
            AttributeCategory::
          }*/
          //let data: AttributeCategory = &data.attributes;



          //let Dimensions(x, y) = data;
          
          //transform.move_right(input_status.input_scale * current_time * x);

          transform.move_right(velocity.vel.x);
        },
        None => (),
      }
    }
  }
}