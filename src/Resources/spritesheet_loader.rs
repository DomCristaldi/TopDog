extern crate amethyst;

use amethyst::{
  prelude::*,
  assets::{
    AssetStorage,
    Handle,
    Loader,
  },
  renderer::{
    ImageFormat,
    SpriteSheet,
    SpriteSheetFormat,
    Texture,
  },
};


pub fn retrieve_spritesheet_handle(world: &mut World) -> Handle<SpriteSheet>
{
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    loader.load(
      "spritesheets/Char1_sprSheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  
  loader.load(
    "spritesheets/Char1_sprSheet.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}