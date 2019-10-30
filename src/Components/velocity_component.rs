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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Velocity2D
{
    //pub vel: math::Vector2<f32>,
    pub vel: math::Vector3<f32>,
}
impl Component for Velocity2D
{
    type Storage = DenseVecStorage<Self>;
}

pub enum Velocity2D_Init
{
    Components(f32, f32, f32),
    Vector(math::Vector3<f32>),
}

impl Default for Velocity2D
{
    fn default() -> Velocity2D
    {
        Velocity2D{
            vel: math::Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Velocity2D
{
    pub fn new(init: Velocity2D_Init) -> Velocity2D
    {
        match init
        {
            Velocity2D_Init::Components(x, y, z) =>
            {
                Velocity2D {
                    vel: math::Vector3::new(x, y, z),
                }
            },
            Velocity2D_Init::Vector(input_vector) =>
            {
                Velocity2D {
                    vel: input_vector,
                }
            },
            _ => Velocity2D::default()
        }
    }

    pub fn MoveTowards(&mut self, target: &Velocity2D, max_magnitude_delta: &f32)
    {
        let to_target: math::Vector3<f32> = target.vel - self.vel;
        
        let mag: f32 = math::Matrix::magnitude(&to_target);
        if (mag == 0.0)
        {
            return;
        }

        let direc: math::Vector3<f32> = math::Matrix::normalize(&to_target);

        //let clamp_max = math::RealField::abs(&max_magnitude_delta);
        let new_mag: f32 = math::clamp(mag, 0.0, max_magnitude_delta.abs());

        self.vel += direc * new_mag;
    }
}



use crate::{
    Editor::Traits::{
        ImguiEditorDisplayable,
    }
};

use amethyst_imgui::{
    imgui,
    imgui::{ im_str, ImString, },
};

//impl ImguiEditorDisplayable<amethyst::prelude::World> for Velocity2D
impl ImguiEditorDisplayable<()> for Velocity2D
{
    fn editor_display_mut(&mut self, ui: &imgui::Ui, world: &())
    {

        let mut v = self.vel.into();
        let confirmed: bool = ui.input_float3(im_str!("Velocity"), &mut v)
        .read_only(false)
        .enter_returns_true(true)
        .build();

        if (confirmed)
        {
            self.vel = math::Vector3::<f32>::from(v);
        }
    }
}