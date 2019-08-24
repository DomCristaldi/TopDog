extern crate amethyst;

use amethyst::{
  assets::{
    Handle,
    Prefab,
  },
  core::{
    Transform,
    Time,
    math,
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
    InputStatusComponent,
    CharacterMovementStateComponent,
    Velocity2D_Init,
    Velocity2D,
  },
};

pub struct CharacterMovementSystem;

impl<'s> System<'s> for CharacterMovementSystem
{
  type SystemData = (
    Read<'s, Time>,
    ReadStorage<'s, InputStatusComponent>,
    ReadStorage<'s, CharacterMovementStateComponent>,
    WriteStorage<'s, Velocity2D>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (time, input_status_components, character_movement_components, mut velocity_components, mut transform_components): Self::SystemData)
  {
    let delta_time = time.delta_seconds();

    for (input_status, character_movement, velocity, transform) in (&input_status_components, &character_movement_components, &mut velocity_components, &mut transform_components).join()
    {
      let delta_vel: f32 = if (input_status.input_scale.abs() > 0.0) && (input_status.input_scale.signum() == velocity.vel.x.signum()) {
        character_movement.accel * delta_time } else {
        -1.0 * character_movement.decel * delta_time };

      let target_speed = input_status.input_scale * character_movement.max_speed;
      
      let vel_init = Velocity2D_Init::Components(target_speed, velocity.vel.y.clone());
      let target_vel: Velocity2D = Velocity2D::new(vel_init);

      // if we're trying to stop, and we would overshoot with a higher delta_vel
      if (target_speed == 0.0) && (delta_vel.abs() > velocity.vel.x.abs())
      {
        velocity.vel = target_vel.vel; // just steal the target velocity, we confirmed it's 0.0
      }
      else
      {
        velocity.MoveTowards(&target_vel, &delta_vel);
      }

      transform.move_right(velocity.vel.x);

      /*println!("{:#?}", character_movement);
      println!("{:#?}", input_status);
      println!("{:#?}", velocity);*/
    }
  }
}