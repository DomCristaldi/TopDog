use amethyst::{
  core::{
    Transform
  },
  core::math,
  ecs::{
    System,
    Entities,
    Join,
    Read,
    ReadStorage,
    WriteStorage,
  },
};

use std::time::{
  Instant,
  Duration,
};

use crate::{
  Components::{
    InputStatusComponent,
    Velocity2D,
    Attributes::{
      CharacterJumpStateComponent,
    },
    Status::{
      JumpStatusComponent,
    },
  },
  Utility::time,
};


pub struct CharacterJumpSystem;

impl CharacterJumpSystem
{
  pub fn calculate_jump_position(progress_normalized: f32, start_pos: f32, max_height: f32) -> f32
  {
    //let delta = (start_pos + max_height) - start_pos;
    //return math::clamp(progress_normalized, 0.0, 1.0) * delta;
    return start_pos + (math::clamp(progress_normalized, 0.0, 1.0) * max_height);
  }
}

impl<'s> System<'s> for CharacterJumpSystem
{
  type SystemData = (
    WriteStorage<'s, JumpStatusComponent>,
    ReadStorage<'s, InputStatusComponent>,
    ReadStorage<'s, CharacterJumpStateComponent>,
    ReadStorage<'s, Transform>,
    WriteStorage<'s, Velocity2D>,
  );

  fn run(&mut self, (mut jump_status_comps, input_status_comps, jump_state_comps, transform_comps, mut velocity_comps): Self::SystemData)
  {
    for (mut jump_status, input_status, jump_state, transform, mut velocity)
      in (&mut jump_status_comps, &input_status_comps, &jump_state_comps, &transform_comps, &mut velocity_comps).join()
    {
      let time_in_jump: Duration = Instant::now() - jump_status.jump_begin_moment;

      let ground_to_apex_duration: Duration =
        Duration::new(jump_state.time_to_max_height_sec.trunc() as u64, 
          (jump_state.time_to_max_height_sec.fract() * f32::powi(10.0, 9)) as u32);

      let progress_to_apex: f32 = time::get_progress_into_duration_normalized(Instant::now(), jump_status.jump_begin_moment, ground_to_apex_duration);

      let desired_height: f32 = CharacterJumpSystem::calculate_jump_position(progress_to_apex, jump_status.jump_begin_loc.y, jump_state.max_jump_height);

      let desired_velocity = desired_height - transform.translation().y;

      velocity.vel.y = desired_velocity;

      if (progress_to_apex >= 1.0)
      {
        jump_status.has_reached_apex = true;
      }
    }
  }
}