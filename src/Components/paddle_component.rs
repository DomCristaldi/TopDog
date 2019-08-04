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

use crate::{
    Resources,
};


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PrefabData)]
pub enum Side
{
    Left,
    Right,
}

pub enum MomentumProfile
{
    Ground(f32),
    Air(f32),
}


#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct PaddleComponent
{
    pub side: Side,
    pub dimensions: Resources::Dimensions,
}

impl Component for PaddleComponent //use derive_new::new;
{
    type Storage = DenseVecStorage<Self>;
}

impl PaddleComponent
{
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<PaddleComponent>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, PaddleComponent>|
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
}