#![allow(warnings)]
/*#![feature(duration_float)]*/

extern crate amethyst;
extern crate amethyst_imgui;

use amethyst::{
    prelude::*,
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{
        InputBundle,
        StringBindings,
    },
    renderer::{
        plugins::{ RenderFlat2D, RenderToWindow, RenderDebugLines },
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir,
};

use amethyst_physics::PhysicsBundle;
use amethyst_nphysics::NPhysicsBackend;

mod Compatability;
mod StaticData;
mod States;
mod Entities;
mod Resources;
mod DataTypes;
mod Systems;
mod Components;
mod Prefabs;
mod Utility;

mod Editor;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources").join("display_config.ron");
    let input_binding_path  = app_root.join("resources").join("bindings_config.ron");
    let assets_dir          = app_root.join("assets");


    let game_data = GameDataBuilder::default()
        .with_system_desc( PrefabLoaderSystemDesc::<Prefabs::PaddlePrefab>::default(), "paddle_loader", &[] )
        .with_system_desc( PrefabLoaderSystemDesc::<Prefabs::CameraPrefabData>::default(), "game_camera_loader", &[] )
        .with_system_desc( PrefabLoaderSystemDesc::<Prefabs::CollidableSurfacePrefab>::default(), "collision_surface_loader", &[] )
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(input_binding_path)?,
        )?
        .with(Systems::CharacterInputSystem, "character_input_system", &["input_system"])
        .with(Systems::CharacterStatusSystem, "character_status_system", &["input_system"])
        .with(Systems::CharacterMovementSystem, "character_movement_system", &["character_input_system", "character_status_system"])
        .with(Systems::CharacterJumpSystem, "character_jump_system", &["character_input_system", "character_status_system"])
        .with(Systems::CharacterFallSystem, "character_fall_system", &["character_input_system", "character_status_system"])
        //.with(Systems::EntityMoverSystem, "entity_mover_system", &["character_movement_system"])

        //.with(Systems::Physics::PhysicalCharacterMoverSystem, "physical_character_mover_sys", &["character_movement_system"])

        .with_bundle(TransformBundle::new())?

        .with_bundle( PhysicsBundle::<f32, NPhysicsBackend>::new()
            .with_pre_physics(Systems::Physics::PhysicalCharacterMoverSystem, String::from("physical_character_mover_sys"), vec![])
        )?

        .with(Systems::Debug::DebugLinesClearer, "debug_lines_clearer", &[])
        .with(Systems::Debug::DebugLineDrawer_Dimensions_System, "debugLinesDrawer_dimensions_system", &["debug_lines_clearer"])
        .with(Systems::Debug::DebugLineDrawer_Colliders_System, "debugLinesDrawer_colliders_system", &["debug_lines_clearer"])
 
        .with(Editor::Systems::ImguiOverlaySystem::default(), "imgui_overlay_system", &[])

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
            )

            .with_plugin(RenderDebugLines::default())

            .with_plugin(amethyst_imgui::RenderImgui::<StringBindings>::default())
        )?
        ;

    let mut game = Application::new(assets_dir, States::Gameplay, game_data)?;

    game.run();

    return Ok(());
}