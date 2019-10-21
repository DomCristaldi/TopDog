use amethyst::{
  core::{ Transform,
    math::{ Point, Point2, Point3, Vector3, },
  },
  ecs::{
    System, SystemData, Join, ReadStorage, WriteStorage,
  },
  renderer::{
    debug_drawing::{ DebugLines, DebugLinesComponent, },
    palette::{ rgb::Srgba, },
  },
};

use crate::{
  Components,
};


pub struct DebugLineDrawer_Dimensions_System;

impl<'a> System<'a> for DebugLineDrawer_Dimensions_System
{
  type SystemData = (
    //Write<'a, DebugLines>,
    WriteStorage<'a, DebugLinesComponent>,
    ReadStorage<'a, Components::Dimensions>,
    ReadStorage<'a, Transform>,
  );

  //fn run(&mut self, (mut debug_lines_resource, dimensions_comps, transform_comps): Self::SystemData)
  fn run(&mut self, (mut debug_lines_comps, dimensions_comps, transform_comps): Self::SystemData)
  {
    for (mut debug_lines, dimensions, transform)
      in (&mut debug_lines_comps, &dimensions_comps, &transform_comps).join()
    {
      //debug_lines.clear();

      let Components::Dimensions(dim_x, dim_y) = dimensions;
      let rect_extents: Vector3<f32> = Vector3::new(dim_x / 2.0, dim_y / 2.0, 0.0);

      let &location: &Vector3<f32> = transform.translation();

      //debug_lines.add_line(
      debug_lines.add_box(
        Point3::from(location + rect_extents),
        Point3::from(location - rect_extents),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
      )
    }

  }
}