//extern crate ncollide2d;

use amethyst::{
  core::math::{ Vector2, Vector3, RealField, },
  assets::{ PrefabData, ProgressCounter },
  derive::PrefabData,
  ecs::{ Component, Entity, VecStorage, },
  error::Error,
};

use amethyst_physics::{
  objects::{ PhysicsHandle, PhysicsTag, PhysicsShapeTag},
  servers::{
    ShapeDesc,
    PhysicsWorld,
  }
};


use serde::{
  Serialize,
  Deserialize,
};


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ColliderAttribute;

impl ColliderAttribute
{
  pub fn new(phys_world: &mut PhysicsWorld<f32>, dimensions: Vector2<f32>) -> PhysicsHandle<PhysicsShapeTag>
  {
    //let desc = ShapeDesc::Cube{Vector3::new(dimensions.x, dimensions.y, 1.0)};
    let desc = ShapeDesc::Cube{
      half_extents: Vector3::new(dimensions.x, dimensions.y, 1.0),
    };

    let shapeHandle = phys_world.shape_server().create(&desc);

    shapeHandle
  }
}

impl Component for ColliderAttribute
{
  type Storage = VecStorage<Self>;
}