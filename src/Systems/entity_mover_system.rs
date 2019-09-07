extern crate specs_physics;

use amethyst::{
  core::{
    Transform,
  },
  core::math,
  ecs::{
    Read,
    ReadStorage,
    WriteStorage,
    System,
    SystemData,
    Entities,
    Join,
  },
  shrev::ReaderId,
};

use specs_physics::{
  events::{ ContactEvent, ContactEvents, ContactType, },
  bodies::PhysicsBody,
  nphysics::{
    algebra::{ Force3, Velocity3, },
  },
};


use crate::{
  Components::{
    Velocity2D,
  }
};

#[derive(Debug, Default)]
pub struct EntityMoverSystem
{
  contact_event_reader: Option<ReaderId<ContactEvent>>
}

impl<'s> System<'s> for EntityMoverSystem
{
  type SystemData = (
    Read<'s, ContactEvents>,
    ReadStorage<'s, Velocity2D>,
    WriteStorage<'s, PhysicsBody<f32>>,
    WriteStorage<'s, Transform>,
  );

  fn setup(&mut self, res: &mut amethyst::ecs::Resources)
  {
    Self::SystemData::setup(res);
    self.contact_event_reader = Some(res.fetch_mut::<ContactEvents>().register_reader());
  }

  fn run(&mut self, (contact_events, velocity_components, mut phys_body_comps, mut transform_components): Self::SystemData)
  {
    for event in contact_events.read(self.contact_event_reader.as_mut().unwrap())
    {
      println!("{:#?}", event);
    }

    for (velocity, phys_bod) in (&velocity_components, &mut phys_body_comps).join()
    {
      let vec: math::Vector3<f32> = math::Vector3::new(0.0, 10.0, 0.0);
      let force_vec: Force3<f32> = Force3::linear(vec);

      //phys_bod.apply_external_force(&force_vec);
      //phys_bod.velocity = Velocity3::linear(velocity.vel.x, velocity.vel.y, velocity.vel.z);
      //phys_bod.velocity = Velocity3::new_with_vectors(velocity.vel, (0.0, 0.0, 0.0));

      //print!("{:#?}", velocity.vel);
    }

    /*for (velocity, transform) in (&velocity_components, &mut transform_components).join()
    {
      transform.prepend_translation(velocity.vel);
    }*/
  }
}