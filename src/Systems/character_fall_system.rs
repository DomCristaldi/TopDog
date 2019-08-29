use amethyst::{
  core::{
    Time,
  },
  core::math,
  ecs::{
    System,
    Join,
    Read,
    ReadStorage,
    WriteStorage,
  },
};

use crate::{
  Components::{
    Attributes,
    Status,
    Velocity2D,
  },
};

pub struct CharacterFallSystem;

impl<'a> System<'a> for CharacterFallSystem
{
  type SystemData = (
    Read<'a, Time>,
    ReadStorage<'a, Attributes::MassAttribute>,
    ReadStorage<'a, Status::FallStatusComponent>,
    WriteStorage<'a, Velocity2D>,
  );

  fn run (&mut self, (time, mass_comps, fall_status_comps, mut velocity_comps): Self::SystemData)
  {
    let delta_time = time.delta_seconds();

    for (mass, fall_status, mut velocity) in (&mass_comps, &fall_status_comps, &mut velocity_comps).join()
    {
      //velocity.vel.y = -0.5;

      velocity.vel.y += 
        math::clamp(
          (fall_status.fall_accel * mass.mass * delta_time).abs(),
          0.0,
          fall_status.max_speed)
        .copysign(-1.0);
    }
  }
}