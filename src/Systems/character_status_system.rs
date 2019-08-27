use amethyst::{
  prelude::{
    World,
  },
  core::{
    Transform,
  },
  core::math::{
    Point,
    Point3,
    Vector3,
  },
  ecs::{
    Read,
    ReadStorage,
    WriteStorage,
    LazyUpdate,
    Join,
    System,
    Entity,
    Entities,
  }
};

use std::{
  convert::*,
  time::Instant,
};

use crate::{
  Components::{
    Status,
    InputStatusComponent,
  },
};


pub struct CharacterStatusSystem;

impl<'a> System<'a> for CharacterStatusSystem
{
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Transform>,
    ReadStorage<'a, InputStatusComponent>,
    ReadStorage<'a, Status::JumpStatusComponent>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self, (entities, transform_comps, input_status_comps, mut jump_status_comps, lazy_updater): Self::SystemData)
  {

    // TODO: ADD JUMP STATUS COMPONENT

    for (entity, transform, input_status, _)
      in (&entities, &transform_comps, &input_status_comps, !&jump_status_comps).join()
    {
      if (input_status.b_wants_jump)
      {
        let location: &Vector3<f32> = transform.translation();
        let location_point: Point3<f32> = Point3::new(location.x, location.y, location.z);

        let new_jump_status = Status::JumpStatusComponent::new(location_point, Instant::now());

        lazy_updater.insert(entity, new_jump_status);
      }
    }

    /*lazy_updater.exec_mut(move |world|
    {
      world.maintain();
    });*/


    //self.entity.wr
    //write_storage.insert(self.entity, Components::Status::JumpStatusComponent);
  }
}