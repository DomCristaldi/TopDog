#![allow(warnings)]

extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir,
};

mod States;
mod Entities;


fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
          RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            
            // RenderFlat2D plugin is used to render entities with a "SpriteRender" copmonent
            .with_plugin(
                RenderFlat2D::default()
            ),
        )?;



    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, States::Gameplay, game_data)?;

    game.run();

    return Ok(());
}