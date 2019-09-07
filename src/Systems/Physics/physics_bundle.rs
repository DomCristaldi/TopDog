use amethyst::{
  assets::Loader,
    core::*,
    ecs::prelude::*,
    input::{Axis, Bindings, Button, InputBundle, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::texture::TextureGenerator,
        light::{Light, SunLight},
        mtl::{Material, TextureOffset},
        palette::rgb::Srgb,
        pass::DrawShadedDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
            mesh::{Normal, Position, TexCoord},
        },
        shape::{Shape as AmethystShape, ShapeUpload},
        types::DefaultBackend,
        Camera, GraphCreator, RenderingSystem, Texture,
    },
    utils::application_root_dir,
    window::{DisplayConfig, ScreenDimensions, Window, WindowBundle},
  /*core::{
    Transform,
    bundle::SystemBundle,
    ecs::DispatcherBuilder,
  },*/
  error::Error,
};

extern crate specs_physics;
use specs_physics::{
  register_physics_systems,
  /*systems::{
    PhysicsStepperSystem,
    SyncBodiesFromPhysicsSystem,
    SyncBodiesToPhysicsSystem,
    SyncCollidersToPhysicsSystem,
    SyncParametersToPhysicsSystem,
  },
  SimplePosition,
  PhysicsBody,*/
  colliders::Shape,
    nalgebra::Vector3,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    parameters::Gravity,
    PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder,
    bodies::*,
};

use derivative::Derivative;

use crate::Systems::Physics::compatibility;


#[derive(Debug, Derivative)]
#[derivative(Default(new="true"))]
pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b,> for PhysicsBundle
{
  fn build(self, builder: &mut DispatcherBuilder<'a, 'b,>) -> Result<(), Error>
  {
    //register_physics_systems::<f32, SimplePosition<f32>>(builder);
    register_physics_systems::<f32, amethyst::core::Transform>(&mut builder);


    /*builder.add(
      SyncBodiesToPhysicsSystem::<f32, SimplePosition<f32>>::default(),
      "sync_bodies_to_physics_system",
      &[]
    );
    builder.add(
      SyncCollidersToPhysicsSystem::<f32, SimplePosition<f32>>::default(),
      "sync_colliders_to_physics_system",
      &[]
    );
    builder.add(
      SyncParametersToPhysicsSystem::<f32>::default(),
      "sync_params_to_physics_system",
      &[]
    );
    builder.add(
      PhysicsStepperSystem::<f32>::default(),
      "physics_stepper_system",
      &[
        "sync_bodies_to_physics_system",
        "sync_colliders_to_physics_system",
        "sync_params_to_physics_system",
      ]
    );
    builder.add(
      SyncBodiesFromPhysicsSystem::<f32, SimplePosition<f32>>::default(),
      "sync_bodies_from_physics_system",
      &["physics_stepper_system"]
    );*/
    // builder.add(
    //   SyncBodiesFromPhysicsSystem::<f32, SimplePosition<f32>>::default(),
    //   "sync_bodies_from_physics_system",
    //   &["physics_stepper_system"]
    // );

    Ok(())
  }
}