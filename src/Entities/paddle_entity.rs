use amethyst::{
  prelude::*,
  error::Error,
  assets::{
    AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, PrefabLoaderSystem, ProgressCounter, RonFormat,
  },
  core::transform::Transform,
  derive::PrefabData,
  ecs::Entity,
  ecs::prelude::{
    Component, DenseVecStorage, WriteStorage ,
  },
  renderer::{
    SpriteRender,
  },
  utils::scene::BasicScenePrefab,
};

use specs_derive::Component;

use specs_physics::{
  colliders::Shape,
  nalgebra::{Isometry3, Vector3,},
  nphysics::{
    object::BodyStatus,
    algebra::Velocity3,
  },
  PhysicsBodyBuilder, PhysicsColliderBuilder, SimplePosition,
};

use serde::{Deserialize, Serialize};

use crate::{
  Components::{
    //CharacterMovementComponent,
    InputStatusComponent,
    PlayerAvatarComponent,
    Velocity2D,
    Velocity2D_Init,
    Dimensions,
    Attributes::{
      CharacterMovementStateComponent,
    },
  },
  Prefabs::{
    PaddlePrefab,
  },
  Resources,
};
/*use crate::{
    
};*/

#[derive(Debug)]
pub struct PaddleEntity;

impl PaddleEntity
{
    pub fn initialize(world: &mut World)
    {
        let prefab_handle: Handle<Prefab<PaddlePrefab>> = PaddlePrefab::retrieve_prefab_handle(world);
        //let prefab_handle: Handle<Prefab<Paddle>> = Resources::DataBase::retrieve_prefab_handle(world, "paddle.ron");

        let mut transform = Transform::default();
        transform.set_translation_xyz(2.0, 50.0, 0.0);

        //Resources::retrieve_spritesheet_handle(world).clone()

        let sprite_render = SpriteRender{
            sprite_sheet: Resources::retrieve_spritesheet_handle(world).clone(),
            sprite_number: 0,
        };

        let collision_shape = Shape::<f32>::Cuboid
        {
          half_extents: Vector3::<f32>::new( 5.0, 5.0, 5.0 ),
        };

        world
            .create_entity()
            .with(prefab_handle.clone())
            .with(transform)
            /*.with(SimplePosition::<f32>(
              Isometry3::<f32>::translation( 2.0, 50.0, 0.0 )
            ))*/
            .with(sprite_render.clone())
            .with(InputStatusComponent::default())
            .with(PlayerAvatarComponent{
                player_index: 0,
            })
            .with(Velocity2D::new(Velocity2D_Init::Components(0.0, 0.0, 0.0)))
            .with(
              PhysicsBodyBuilder::<f32>::from( BodyStatus::Kinematic )
              //.velocity(Velocity3::linear(0.0, -10.0, 0.0))
              .build()
            )
            .with(
              //PhysicsColliderBuilder::<f32>::from( Shape::Rectangle ( 5.0, 5.0, 5.0 ) )
              PhysicsColliderBuilder::<f32>::from(collision_shape)
              .build()
            )
            .build();
    }
}