use amethyst::{
  prelude::{ World, WorldExt, Builder, },

  assets::{ Handle, Prefab, PrefabData, ProgressCounter, },

  core::{ Transform, },
  core::math::{ Vector3, },

  ecs::{ /*World,*/ Entity, },

  renderer::{
    debug_drawing::DebugLinesComponent,
  },
};

use amethyst_physics::{
    prelude::*,
    /*PhysicsWorld,
    servers::{
        ShapeDesc, RigidBodyDesc, BodyMode,
    },*/
};

use crate::{
  Components::{ Dimensions, },
  Prefabs::{ CollidableSurfacePrefab },
};


#[derive(Debug)]
pub struct CollisionSurfaceEntity;

impl CollisionSurfaceEntity
{
  pub fn initialize(world: &mut World)
  {
    let prefab_handle: Handle<Prefab<CollidableSurfacePrefab>>
      = CollidableSurfacePrefab::retieve_prefab_handle(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 50.0, 0.0);

    let rigid_body_comp = {
            let mut rb_desc = RigidBodyDesc::default();
            rb_desc.mode = BodyMode::Static;
            rb_desc.mass = 1.0;
            rb_desc.bounciness = 0.0;
            rb_desc.friction = 0.05;

            rb_desc.lock_rotation_z = true;
            rb_desc.lock_translation_z = true;

            let physics_world = world.fetch::<PhysicsWorld<f32>>();

            physics_world.rigid_body_server().create(&rb_desc)
        };

        let shape_comp = {
            let desc = ShapeDesc::Cube {
                half_extents: Vector3::new(50.0 / 2.0, 20.0 / 2.0, 1.0),
            };

            let physics_world = world.fetch::<PhysicsWorld<f32>>();

            physics_world.shape_server().create(&desc)
        };

    world
      .create_entity()
      .with(prefab_handle.clone())
      //.with(transform)

      //.with(rigid_body_comp)
      .with(shape_comp)

      .with(DebugLinesComponent::default())

      .build();
  }
}