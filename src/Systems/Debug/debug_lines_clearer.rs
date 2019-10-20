use amethyst::{
  ecs::{
    Entities, System, SystemData, Join, ReadStorage, WriteStorage, DenseVecStorage,
  },
  renderer::{
    debug_drawing::{ DebugLines, DebugLinesComponent, },
  },
};

pub struct DebugLinesClearer;

impl<'a> System<'a> for DebugLinesClearer
{
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, DebugLinesComponent>,
  );

  //fn run(&mut self, (mut debug_lines_resource, dimensions_comps, transform_comps): Self::SystemData)
  fn run(&mut self, (entities, mut debug_lines_comps): Self::SystemData)
  {
    for mut debug_lines in (&mut debug_lines_comps).join()
    {
      debug_lines.clear();
    }
  }
}