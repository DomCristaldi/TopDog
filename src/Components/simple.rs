use amethyst::{
    error::Error,
    assets::{
        PrefabData,
    },
    core::math,
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
#[serde(deny_unknown_fields)]
pub struct Dimensions(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Position(pub f32, pub f32, pub f32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Velocity2D
{
    pub vel: math::Vector2<f32>,
}
impl Component for Velocity2D
{
    type Storage = DenseVecStorage<Self>;
}

//pub struct Velocity2D(pub f32, pub f32);

impl Default for Velocity2D
{
    fn default() -> Velocity2D
    {
        Velocity2D
        {
            vel: math::Vector2::new(0.0, 0.0),
        }
    }
}


impl Velocity2D
{
    pub fn new(&mut self, x: f32, y: f32) -> Velocity2D
    {
        Velocity2D{
            vel: math::Vector2::new(x, y),
        }
    }

    pub fn MoveTowards(&mut self, target: &Velocity2D, max_magnitude_delta: &f32)
    {
        let to_target: math::Vector2<f32> = target.vel - self.vel;
        
        let direc: math::Vector2<f32> = math::Matrix::normalize(&to_target);
        let mag: f32 = math::Matrix::magnitude(&to_target);

        let new_mag: f32 = math::clamp(max_magnitude_delta.clone(), 0.0, mag);

        self.vel = direc * new_mag;
    }
}