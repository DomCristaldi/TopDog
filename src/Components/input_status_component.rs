extern crate amethyst;

use amethyst::{
    prelude::*,
    error::Error,
    core::transform::Transform,
    core::Time,
    ecs::prelude::{
      Component,
      DenseVecStorage,
      WriteStorage 
      },
};

#[derive(Debug)]
pub struct InputStatusComponent
{
  pub input_scale: f32,
  pub b_wants_jump: bool,
  pub b_wants_stomp: bool,
}

impl Component for InputStatusComponent
{
  type Storage = DenseVecStorage<Self>;

}