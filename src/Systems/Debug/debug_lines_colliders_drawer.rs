use amethyst::{
  ecs::{ System, SystemData, Join, ReadStorage, ReadExpect, WriteStorage },
  renderer::debug_drawing::{ DebugLinesComponent, },
};

use amethyst_physics::{
  objects::{ PhysicsHandle, PhysicsTag, PhysicsShapeTag},
  servers::{
    ShapeDesc,
    PhysicsWorld,
  }
};

use crate::{
  Components,
};

pub struct DebugLineDrawer_Colliders_System;

impl<'a> System<'a> for DebugLineDrawer_Colliders_System
{
  type SystemData = (
    WriteStorage<'a, DebugLinesComponent>,
    ReadExpect<'a, PhysicsWorld<f32>>,
    ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
    //ReadStorage<'a, Components::Attributes::ColliderAttribute>,
  );

  fn run(&mut self, (mut debug_lines_comps, phys_world, phys_shape_handles): Self::SystemData)
  {
    for (mut debug_lines, phys_shape)
      in (&mut debug_lines_comps, &phys_shape_handles).join()
    {
      //phys_shape.
    }
  }
}