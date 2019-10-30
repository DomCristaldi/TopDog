use std::sync::{ Arc, Mutex, };

use amethyst::{
  prelude::*,
  core::math::{
    Vector3,
  },
  ecs::{
    System, SystemData, ReadExpect, ReadStorage, WriteStorage, Join,
  },
};

use amethyst_imgui::{
  //ImguiContextWrapper,
  imgui,
  imgui::{ im_str, ImString, },
};

use amethyst_physics::{
  prelude::*,
};

use crate::{
  Components::{
    Velocity2D,
  },
  Editor::Traits::ImguiEditorDisplayable,
};


pub struct ImguiOverlaySystem
{
  corner: i32,
  open: bool,
  test_float: f32,
}

impl Default for ImguiOverlaySystem
{
  fn default() -> Self
  {
    ImguiOverlaySystem
    {
      corner: 0,
      open: true,
      test_float: 0.0,
    }
  }
}

impl<'a> System<'a> for ImguiOverlaySystem
{
  type SystemData = (
    ReadExpect<'a, PhysicsWorld<f32>>,
    WriteStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
    WriteStorage<'a, Velocity2D>,
  );

  fn run(&mut self, (sys_data): Self::SystemData)
  {
    amethyst_imgui::with(|ui|
    {
      let mut open = true;

      let title = im_str!("Example: Simple overlay");

      let mut window = imgui::Window::new(&title)
        .bg_alpha(0.35)
        .movable(true)
        .always_auto_resize(true)
        .save_settings(false)
        .focus_on_appearing(false)
        .no_nav()
        .opened(&mut open);

      window.build(ui, ||{

        let (phys_world, mut rigBodTags, mut vel2D_storage) = sys_data;
        for (mut rig_bod, mut vel2d) in (&mut rigBodTags, &mut vel2D_storage).join()
        {
          ui.text("test window");

          rig_bod.editor_display_mut(&ui, &phys_world);

          vel2d.editor_display_mut(&ui, &());
        }
      });

    });



  }
}