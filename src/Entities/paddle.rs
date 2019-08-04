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
        SpriteRender,
    },
    utils::scene::BasicScenePrefab,
};

use serde::{Deserialize, Serialize};
use specs_derive::Component;

/*use crate::{
    Resources::Dimensions,
};*/
use crate::{
    Resources,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PrefabData)]
pub enum Side
{
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct Paddle
{
    pub side: Side,
    pub dimensions: Resources::Dimensions,
}

impl Component for Paddle//use derive_new::new;
{
    type Storage = DenseVecStorage<Self>;
}

impl Paddle
{
    pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<Paddle>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, Paddle>|
            {
                loader.load(
                    "paddle.ron",
                    RonFormat,
                    ()
                )
            }
        );

        return prefab_handle;
    }


    pub fn initialize(world: &mut World)
    {
        let prefab_handle: Handle<Prefab<Paddle>> = Paddle::retrieve_prefab_handle(world);
        //let prefab_handle: Handle<Prefab<Paddle>> = Resources::DataBase::retrieve_prefab_handle(world, "paddle.ron");

        let mut left_transform = Transform::default();
        let mut right_transform = Transform::default();

        left_transform.set_translation_xyz(2.0, 50.0, 0.0);

//        Resources::retrieve_spritesheet_handle(world).clone()

        let sprite_render = SpriteRender{
            sprite_sheet: Resources::retrieve_spritesheet_handle(world).clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(prefab_handle.clone())
            .with(left_transform)
            .with(sprite_render.clone())
            .build();
    }
}