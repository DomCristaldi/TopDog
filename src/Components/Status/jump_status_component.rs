use amethyst::{
  core::{
    Time,
  },
  ecs::{
    Component,
    DenseVecStorage,
    WriteStorage,
  }
};

use std::{
  time::Instant,
};

pub struct JumpStatusComponent
{
  take_off_moment: Instant,
  has_reached_apex: bool,
}

impl JumpStatusComponent
{
  pub fn new(jump_begin: Instant) -> JumpStatusComponent
  {
    JumpStatusComponent{
      take_off_moment: jump_begin,
      has_reached_apex: false,
    }
  }
}

impl Component for JumpStatusComponent
{
  type Storage = DenseVecStorage<Self>;
}