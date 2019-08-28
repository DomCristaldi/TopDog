use amethyst::{
  ecs::{
    System,
    Join,
    ReadStorage,
    WriteStorage,
  },
};

use crate::{
  Components::{
    Status::FallStatusComponent,
    Velocity2D,
  },
};

pub struct CharacterFallSystem;

impl<'a> System<'a> for CharacterFallSystem
{
  type SystemData = (
    ReadStorage<'a, FallStatusComponent>,
    WriteStorage<'a, Velocity2D>,
  );

  fn run (&mut self, (fall_status_comps, mut velocity_comps): Self::SystemData)
  {
    for (fall_status, mut velocity) in (&fall_status_comps, &mut velocity_comps).join()
    {
      velocity.vel.y = -0.5;
    }
  }
}