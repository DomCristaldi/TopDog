use amethyst::{
  prelude::Builder,
  core::Transform,
  ecs::{
    Entity,
    Component,
    DenseVecStorage,
    WriteStorage,
    World,
  },
};

use specs_physics::{
  colliders::Shape,
  nalgebra::{ Vector3, },
  nphysics::{
    object::BodyStatus,
  },
  PhysicsBodyBuilder, PhysicsColliderBuilder,
};

#[derive(Debug)]
pub struct GroundEntity;

impl GroundEntity
{
  pub fn initialize(world: &mut World)
  {
    let mut transform = Transform::default();

    transform.set_translation_xyz(0.0, -25.0, 0.0);

    let collision_shape = Shape::<f32>::Cuboid
    {
      half_extents: Vector3::<f32>::new( 50.0, 20.0, 5.0 ),
    };

    world
      .create_entity()
      .with(transform)
      .with(
        PhysicsBodyBuilder::<f32>::from( BodyStatus::Static )
        .build()
      )
      .with(
        PhysicsColliderBuilder::<f32>::from( collision_shape )
        .build()
      )
      .build();
  }
}