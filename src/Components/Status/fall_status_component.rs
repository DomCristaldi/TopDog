use amethyst::{
  core::{
    Time,
  },
  core::math::Point3,
  ecs::{
    Component,
    DenseVecStorage,
    NullStorage,
    WriteStorage,
    FlaggedStorage,
  }
};

extern crate derivative;

use derivative::Derivative;


#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FallStatusComponent
{
  #[derivative(Default(value="0.0"))]
  fall_accel: f32,

  #[derivative(Default(value="0.0"))]
  max_speed: f32,
}

/*impl Default for FallStatusComponent
{
  fn default() -> FallStatusComponent
  {
    FallStatusComponent
    {
      fall_accel: 0.0,
      max_speed: 0.0,
    }
  }
}*/

impl Component for FallStatusComponent
{
  type Storage = DenseVecStorage<Self>;
}