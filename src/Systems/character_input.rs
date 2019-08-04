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

pub struct CharacterInpoutSystem;

impl<'s> System<'s> for CharacterInpoutSystem
{
  type SystemData = (
    WriteStorage<'s, Transform>,
    //ReadStorage<'s, >
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut transforms, input): Self::SystemData)
  {
    //for (transform) in 
    for transform in transforms.join()
    {
      // TODO: account for different players via index on characters

      match input.axis_value("paddle_moveRightLeft")
      {
        Some(movement_input) => {
          println!("Movement: {}", movement_input);
        },
        None => {
          println!("no input found");
        },
      }

      /*let Some(movement_input) = input.axis_value("paddle_moveRightLeft");

      if (movement_input != 0.0)
      {
        match(movement_input)
        
      }*/
    }
  }
}