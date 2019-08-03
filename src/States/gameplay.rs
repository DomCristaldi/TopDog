use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};

use crate::{
    Entities::Paddle,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;


fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers the whole arena and (0, 0) is in the bootom left
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world = data.world;

        initialize_camera(world);

        Paddle::initialize_paddles(world);
        //Paddle::load_prefab(world);
    }
}