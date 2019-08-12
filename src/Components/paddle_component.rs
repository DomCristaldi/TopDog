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

//use derivative::Derivative;

use crate::{
    Resources,
};


/*#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PrefabData)]
pub enum Side
{
    Left,
    Right,
}*/

/*#[derive(Clone, Copy, Component, Debug, Derivative, Deserialize, Serialize, PrefabData)]
#[derivative(Default)]
#[prefab(Component)]
#[storage(VecStorage)]
pub enum MomentumProfile
{
    #[derivative(Default)]
    Ground(f32),
    Air(f32),
}*/


/*
#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct PaddlePrefab
{
    // pub side: Side,
    pub dimensions: Resources::Dimensions,
}

/*impl Component for PaddleComponent //use derive_new::new;
{
    type Storage = DenseVecStorage<Self>;
}*/

impl PaddlePrefab
{
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<PaddlePrefab>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, PaddlePrefab>|
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
*/