use amethyst_physics::{
  prelude::*,
  objects::{
    PhysicsHandle,
    PhysicsRigidBodyTag,
  }
};



use crate::Editor::{
  Traits::{
    ImguiEditorDisplayable,
  },
};

pub use amethyst_imgui::{
  imgui,
  imgui::{ im_str, },
};

impl ImguiEditorDisplayable<PhysicsWorld<f32>> for PhysicsHandle<PhysicsRigidBodyTag>
{
  fn editor_display(&self, ui: &imgui::Ui, world: &PhysicsWorld<f32>)
  {
    let mut linear_vel = world.rigid_body_server().linear_velocity(self.get()).into();
          ui.input_float3(im_str!("Linear Velocity"), &mut linear_vel)
          .read_only(true)
          .build();
  }

   
}