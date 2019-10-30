use amethyst::{
  ecs::{ System, SystemData, ReadStorage, ReadExpect, Join,}
};

use amethyst_physics::{
  prelude::*,
  /*prelude::PhysicsRigidBodyTag,
  prelude::{
    PhysicsHandle, PhysicsTime, PhysicsWorld,
  },*/
};

use crate::{
  Components::{
    CharacterTag,
    Velocity2D,
    InputStatusComponent,
  }
};

pub struct PhysicalCharacterMoverSystem;

impl<'a> System<'a> for PhysicalCharacterMoverSystem
{
  type SystemData = (
    ReadExpect<'a, PhysicsWorld<f32>>,
    ReadExpect<'a, PhysicsTime>,
    ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
    ReadStorage<'a, CharacterTag>,
    ReadStorage<'a, InputStatusComponent>,
    ReadStorage<'a, Velocity2D>,
  );

  fn run(&mut self, sys_data: Self::SystemData)
  {
    let (phys_world, phys_time, rig_bod_handles, character_tags, input_statuses, vel2d_comps) = sys_data;

    for (rig_bod, vel) in (&rig_bod_handles, &vel2d_comps).join()
    {
      //phys_world.rigid_body_server().apply_impulse(rig_bod.get(), &vel.vel);

      
      phys_world.rigid_body_server().set_linear_velocity(rig_bod.get(), &vel.vel);
    }
  }
}