use amethyst::{
  core::Transform,
  ecs::{
    System,
    Join,
    Write,
    ReadStorage,
  },
  renderer::{
    debug_drawing::{
      DebugLines,
    },
    palette::Srgba,
  }
};

use specs_physics::{
  //bodies::PhysicsBody,
  colliders::{ PhysicsCollider, Shape, },
  nalgebra::Vector3,
};

pub struct DebugDrawColliderSystem;

impl<'a> System<'a> for DebugDrawColliderSystem
{
  type SystemData = (
    Write<'a, DebugLines>,
    ReadStorage<'a, Transform>,
    ReadStorage<'a, PhysicsCollider<f32>>,
  );

  fn run (&mut self, (mut debug_lines_writer, transform_comps, physics_colliders): Self::SystemData)
  {
    for (transform, collider) in ( &transform_comps, &physics_colliders ).join()
    {
      match collider.shape
      {
        /*Shape::Circle(n) => {
          
        },*/
        //Shape::Rectangle(rect_x, rect_y, rect_z) => {
        Shape::Cuboid{half_extents} => {

          let rect_x = half_extents.x;
          let rect_y = half_extents.y;
          let rect_z = half_extents.z;

          // TODO: finish debug drawing of collider
          let trans_x: f32 = transform.translation().x;
          let trans_y: f32 = transform.translation().y;
          let trans_z: f32 = transform.translation().z;

          let left_x = trans_x - rect_x;
          let right_x = trans_x + rect_x;
          let top_y = trans_y + rect_y;
          let bottom_y = trans_y - rect_y;

          let line_color: Srgba = Srgba::new(1.0, 0.0, 0.0, 1.0);

          debug_lines_writer.draw_line(
            [left_x, top_y, trans_z].into(),
            [right_x, top_y, trans_z].into(),
            line_color,
          );

          debug_lines_writer.draw_line(
            [left_x, bottom_y, trans_z].into(),
            [right_x, bottom_y, trans_z].into(),
            line_color,
          );

          debug_lines_writer.draw_line(
            [left_x, top_y, trans_z].into(),
            [left_x, bottom_y, trans_z].into(),
            line_color
          );

          debug_lines_writer.draw_line(
            [right_x, top_y, trans_z].into(),
            [right_x, bottom_y, trans_z].into(),
            line_color
          );


          //let thing: f32 = transform.translation().y;
        },
        
        _ => (),
      }

    }

    //debug_lines_writer.draw_line()
  }
}