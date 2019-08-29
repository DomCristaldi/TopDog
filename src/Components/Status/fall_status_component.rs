use amethyst::{
  core::{
    Time,
  },
  core::math::Vector3,
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
  #[derivative(Default(value="9.8"))]
  pub fall_accel: f32,

  #[derivative(Default(value="5.0"))]
  pub max_speed: f32,

  // Should handle with a proper Gravity volume, or global Gravity resource
  /*#[derivative(Default(value = "Vector3::new(0.0, -1.0, 0.0)"))]
  pub fall_direc: Vector3<f32>,*/
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