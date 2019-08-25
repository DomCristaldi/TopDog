use amethyst::{
  core::{
    Transform,
  },
  ecs::{
    ReadStorage,
    WriteStorage,
    System,
    Entities,
    Join,
  }
};

use crate::{
  Components::{
    Velocity2D,
  }
};

pub struct EntityMoverSystem;

impl<'s> System<'s> for EntityMoverSystem
{
  type SystemData = (
    ReadStorage<'s, Velocity2D>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (velocity_components, mut transform_components): Self::SystemData)
  {
    for (velocity, transform) in (&velocity_components, &mut transform_components).join()
    {
      transform.prepend_translation(velocity.vel);
    }
  }
}