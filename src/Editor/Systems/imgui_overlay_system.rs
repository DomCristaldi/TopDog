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
    //ReadExpect<'a, Arc<Mutex<ImguiContextWrapper>>>,
    ReadExpect<'a, PhysicsWorld<f32>>,
    WriteStorage<'a, Velocity2D>,
    ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
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
        //.movable(corner == -1)
        //.no_decoration()
        .always_auto_resize(true)
        .save_settings(false)
        .focus_on_appearing(false)
        .no_nav()
        .opened(&mut open);

      /*if corner != -1
      {
        window = window
          .position(window_pos, imgui::Condition::Always)
          .position_pivot(window_pos_pivot);
      }*/

      window.build(ui, ||{

        let (phys_world, mut vel2D_storage, rigBodTags) = sys_data;
        for (mut vel2d, rig_bod) in (&mut vel2D_storage, &rigBodTags).join()
        {
          ui.text("test window");
          ui.text("more text");


          let mut flTest = 5.0;
          ui.input_float(im_str!("float"), &mut self.test_float).build();

          /*let mut linear_vel = phys_world.rigid_body_server().linear_velocity(rig_bod.get()).into();
          ui.input_float3(im_str!("Linear Velocity"), &mut linear_vel).build();*/

          rig_bod.editor_display(&ui, &phys_world);

          vel2d.editor_display_mut(&ui, &());

        }

        /*ui.drag_float(im_str!("drag float"), &mut self.test_float);
        ui.text("test window");*/
        //ui.color_picker()


      });

    });



  }
}