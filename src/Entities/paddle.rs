extern crate amethyst;

use amethyst::{
    prelude::*,
    assets::{PrefabLoader, PrefabLoaderSystem, RonFormat},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    utils::scene::BasicScenePrefab,
};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side
{
    Left,
    Right,
}

pub struct Paddle
{
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle
{
    /*fn new(side: Side) -> Paddle
    {
        Paddle
        {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }*/

    fn new(side: Side, width: f32, height: f32) -> Paddle
    {
        Paddle { side, width, height, }
    }
}

impl Component for Paddle
{
    type Storage = DenseVecStorage<Self>;
}

type PaddlePrefabData = BasicScenePrefab<(f32, f32)>;

fn initialize_paddles(world: &mut World)
{

    /*let app_root = application_root_dir()?;
    let asset_root = app_root.join("assets");
    let paddle_asset_path = asset_root.join("paddle.rs");*/

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    //let loader = &world.read_resource::<Loader>();

    let prefab_handle = world.exec(|loader: PrefabLoader<'_, PaddlePrefabData>|
    {
        return loader.load("paddle.ron", RonFormat, ());
    });



    //let y = ARENA

    //


    /*let prefab_handle = {
        let loader = world.read_resource::<Loader>();

        let paddles = &world.read_resource();
        let paddleData: Handle<RonFormat> = loader.load("paddle.ron", RonFormat, (), paddles);

        return paddleData;
    };*/

    world
        .create_entity()
        .with(Paddle::new(Side::Left, 4.0, 16.0))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right, 4.0, 16.0))
        .with(right_transform)
        .build();
}