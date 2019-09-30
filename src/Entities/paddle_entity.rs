extern crate amethyst;

use amethyst::{
    prelude::*,
    error::Error,
    assets::{
        AssetStorage,
        Handle,
        Prefab,
        PrefabData,
        PrefabLoader,
        PrefabLoaderSystem,
        ProgressCounter,
        RonFormat
        },
    core::transform::Transform,
    derive::PrefabData,
    ecs::Entity,
    ecs::prelude::{
        Component,
        DenseVecStorage,
        WriteStorage 
        },
    renderer::{
        SpriteRender, debug_drawing::DebugLinesComponent,
    },
    utils::scene::BasicScenePrefab,
};

use serde::{Deserialize, Serialize};
use specs_derive::Component;

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

        world
            .create_entity()
            .with(prefab_handle.clone())
            .with(transform)
            .with(sprite_render.clone())
            .with(InputStatusComponent::default())
            .with(PlayerAvatarComponent{
                player_index: 0,
            })
            .with(Velocity2D::new(Velocity2D_Init::Components(0.0, 0.0, 0.0)))

            .with(DebugLinesComponent::default())

            .build();
    }
}