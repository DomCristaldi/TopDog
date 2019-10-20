use amethyst::{
    error::Error,
    assets::{
        PrefabData,
    },
    core::math,
    derive::PrefabData,
    ecs::{
        Entity, Component, DenseVecStorage, WriteStorage,
    },
    utils::scene::BasicScenePrefab,
};

use serde::{Deserialize, Serialize};
//use specs_derive::Component;


#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Dimensions(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Position(pub f32, pub f32, pub f32);