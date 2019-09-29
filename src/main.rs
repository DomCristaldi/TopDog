#![allow(warnings)]
/*#![feature(duration_float)]*/

extern crate amethyst;

use amethyst::{
    prelude::*,
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{
        InputBundle,
        StringBindings,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir,
};

mod StaticData;
mod States;
mod Entities;
mod Resources;
mod DataTypes;
mod Systems;
mod Components;
mod Prefabs;
mod Utility;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources").join("display_config.ron");
    let input_binding_path  = app_root.join("resources").join("bindings_config.ron");
    let assets_dir          = app_root.join("assets");


    let game_data = GameDataBuilder::default()
        .with_system_desc( PrefabLoaderSystemDesc::<Prefabs::PaddlePrefab>::default(), "paddle_loader", &[] )
        .with_system_desc( PrefabLoaderSystemDesc::<Prefabs::CameraPrefabData>::default(), "game_camera_loader", &[] )
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(input_binding_path)?,
        )?
        .with(Systems::CharacterInputSystem, "character_input_system", &["input_system"])
        .with(Systems::CharacterStatusSystem, "character_status_system", &["input_system"])
        .with(Systems::CharacterMovementSystem, "character_movement_system", &["character_input_system", "character_status_system"])
        .with(Systems::CharacterJumpSystem, "character_jump_system", &["character_input_system", "character_status_system"])
        .with(Systems::CharacterFallSystem, "character_fall_system", &["character_input_system", "character_status_system"])
        .with(Systems::EntityMoverSystem, "entity_mover_system", &["character_movement_system"])
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
        )?
        .with_bundle(TransformBundle::new())?
        ;

    let mut game = Application::new(assets_dir, States::Gameplay, game_data)?;

    game.run();

    return Ok(());
}