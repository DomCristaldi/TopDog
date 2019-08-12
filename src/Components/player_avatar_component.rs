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


pub struct PlayerAvatarComponent
{
  pub player_index: u8,
}

impl Component for PlayerAvatarComponent
{
  type Storage = DenseVecStorage<Self>;

}