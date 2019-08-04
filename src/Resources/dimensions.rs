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
    utils::scene::BasicScenePrefab,
};

use serde::{Deserialize, Serialize};
use specs_derive::Component;


#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Dimensions(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Position(pub f32, pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Velocity2D(pub f32, pub f32);