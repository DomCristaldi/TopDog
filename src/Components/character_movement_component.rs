extern crate amethyst;

use amethyst::{
    prelude::*,
    error::Error,
    core::transform::Transform,
    ecs::prelude::{
      Component,
      DenseVecStorage,
      WriteStorage 
      },
};


pub struct CharacterMovementComponent
{

}

impl Component for CharacterMovementComponent
{
      type Storage = DenseVecStorage<Self>;

}