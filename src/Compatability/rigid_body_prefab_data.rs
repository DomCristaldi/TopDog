use amethyst_physics::{
  //objects::{ CollisionGroup, },
  servers::{ BodyMode, RigidBodyDesc, },
};

use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize)]
#[serde(remote = "BodyMode")]
enum BodyMode_SerDe
{
    Disabled,
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "RigidBody")]
pub struct RigBodyDesc_PrefabData
{
    /// Body mode
    #[serde(with = "BodyMode_SerDe")]
    pub mode: BodyMode,
    /// Body mass
    pub mass: Option<f32>,
    /// Body friction range 0 - 1
    pub friction: Option<f32>,
    /// Body bounciness range 0 - 1
    pub bounciness: Option<f32>,
    /// Collision Groups this Rigid Body belong.
    /*pub belong_to: Vec<CollisionGroup>,
    /// Collide with groups.
    pub collide_with: Vec<CollisionGroup>,*/
    /// Lock body translation along X
    pub lock_translation_x: Option<bool>,
    /// Lock body translation along Y
    pub lock_translation_y: Option<bool>,
    /// Lock body translation along Z
    pub lock_translation_z: Option<bool>,

    /// Lock body rotation along X
    pub lock_rotation_x: Option<bool>,
    /// Lock body rotation along Y
    pub lock_rotation_y: Option<bool>,
    /// Lock body rotation along Z
    pub lock_rotation_z: Option<bool>,
}

impl Into<RigidBodyDesc<f32>> for RigBodyDesc_PrefabData
{
  fn into(self) -> RigidBodyDesc<f32>
  {
    let mut rb_desc = RigidBodyDesc::default();

    rb_desc.mode = self.mode;

    macro_rules! extract_data{
      ( $x:ident ) =>
      {
        match self.$x
        {
          Some(val) => rb_desc.$x = val,
          _ => (),
        }
      };
    }

    extract_data!(mass);
    extract_data!(bounciness);
    extract_data!(friction);

    extract_data!(lock_translation_x);
    extract_data!(lock_translation_y);
    extract_data!(lock_translation_z);

    extract_data!(lock_rotation_x);
    extract_data!(lock_rotation_y);
    extract_data!(lock_rotation_z);

    rb_desc
  }
}