extern crate amethyst;

use amethyst::{
  core::{
    Transform,
  },
  ecs::{
    Join,
    Read,
    ReadStorage,
    WriteStorage,
    System,
  },
  input::{
    InputHandler,
    StringBindings,
  },
};

use crate::{
  Components::{
    InputStatusComponent,
    PlayerAvatarComponent,
    Status::{
      JumpStatusComponent,
    }
  },
};

pub struct CharacterInputSystem;

impl<'s> System<'s> for CharacterInputSystem
{
  type SystemData = (
    Read<'s, InputHandler<StringBindings>>,
    ReadStorage<'s, PlayerAvatarComponent>,
    WriteStorage<'s, InputStatusComponent>,
  );

  fn run(&mut self, (device_input, player_avatar_components, mut input_status_components): Self::SystemData)
  {
    for (player_avatar, input_status) in (&player_avatar_components, &mut input_status_components).join()
    {
      // TODO: account for different players via index on characters

      match device_input.axis_value("moveLeftRight")
      {
        Some(movement_input) => {
          input_status.input_scale = movement_input;
        },
        None => (),
      }

      match device_input.action_is_down("jump")
      {
        Some(jump_input) =>
        {
          input_status.b_wants_jump = jump_input;
        },
        None => (),
      }

      match device_input.action_is_down("stomp")
      {
        Some(stomp_input) =>
        {
          input_status.b_wants_stomp = stomp_input;
        },
        None => (),
      }

      //println!("Movement: {:#?}", input_status);
    }
  }
}