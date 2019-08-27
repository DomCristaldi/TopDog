use amethyst::{
  core::{
    Time,
  },
  core::math::Point3,
  ecs::{
    Component,
    DenseVecStorage,
    WriteStorage,
    FlaggedStorage,
  }
};

use std::{
  time::Instant,
};

pub struct JumpStatusComponent
{
  pub jump_begin_loc: Point3<f32>,
  pub jump_begin_moment: Instant,
  pub has_reached_apex: bool,
}

impl JumpStatusComponent
{
  pub fn new(where_pos: Point3<f32>, when_time: Instant) -> JumpStatusComponent
  {
    JumpStatusComponent{
      jump_begin_loc: where_pos,
      jump_begin_moment: when_time,
      has_reached_apex: false,
    }
  }
}

impl Component for JumpStatusComponent
{
  //type Storage = DenseVecStorage<Self>;
  type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}