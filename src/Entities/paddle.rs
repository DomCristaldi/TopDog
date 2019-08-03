extern crate amethyst;

use amethyst::{
    prelude::*,
    error::Error,
    assets::{
        AssetStorage,
        Handle,
        Prefab,
        PrefabData,
        PrefabLoader,
        PrefabLoaderSystem,
        ProgressCounter,
        RonFormat
        },
    core::transform::Transform,
    derive::PrefabData,
    ecs::Entity,
    ecs::prelude::{
        Component,
        DenseVecStorage,
        WriteStorage 
        },
    utils::scene::BasicScenePrefab,
};


//use derive_new::new;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

//type EntityPrefab = Prefab<Paddle>

/*pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;*/


#[derive(Clone, Copy, Debug, Default, Component, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Dimensions(pub f32, pub f32);


/*#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CharacterPrefab()
{

}

impl<'a> PrefabData<'a> for CharacterPrefab
{
    type SystemData = WriteStorage<'a, CharacterPrefab>;

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<Self::Result, Error> {
        system_data.insert(entity, system_data).map(|_| ())?;
        return Ok(());
    }
}*/



#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PrefabData)]
pub enum Side
{
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, PrefabData)]
pub struct Paddle
{
    pub side: Side,
    pub dimensions: Dimensions,
    /*pub side: Side,
    pub width: f32,
    pub height: f32,*/
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

    /*fn new(side: Side, width: f32, height: f32) -> Paddle
    {
        Paddle { side, width, height, }
    }*/
}

impl Component for Paddle
{
    type Storage = DenseVecStorage<Self>;
}

//type PaddlePrefabData = BasicScenePrefab<(f32, f32)>;

impl Paddle
{
    pub fn load_prefab(world: &mut World) -> Handle<Prefab<Paddle>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, Paddle>|
            {
                loader.load(
                    "paddle.ron",
                    RonFormat,
                    ()
                )
            }
        );

        return prefab_handle;
    }


    pub fn initialize_paddles(world: &mut World)
    {
        let prefab_handle: Handle<Prefab<Paddle>> = Paddle::load_prefab(world);

        /*let mut prefabData: Option<&Prefab<Paddle>>;
        {
            let prefab_assets = world.read_resource::<AssetStorage<Prefab<Paddle>>>();

            let prefab = prefab_assets.get(&prefab_handle);

            prefabData = prefab;
        }*/

        /*let app_root = application_root_dir()?;
        let asset_root = app_root.join("assets");
        let paddle_asset_path = asset_root.join("paddle.rs");*/

        let mut left_transform = Transform::default();
        let mut right_transform = Transform::default();

        left_transform.set_translation_xyz(2.0, 50.0, 0.0);

        world
            .create_entity()
            .with(prefab_handle.clone())
            .with(left_transform)
            .build();

        //let loader = &world.read_resource::<Loader>();

        /*let prefab_handle = world.exec(|loader: PrefabLoader<'_, PaddlePrefabData>|
        {
            return loader.load("paddle.ron", RonFormat, ());
        });*/

        //let y = ARENA

        //


        /*let prefab_handle = {
            let loader = world.read_resource::<Loader>();

            let paddles = &world.read_resource();
            let paddleData: Handle<RonFormat> = loader.load("paddle.ron", RonFormat, (), paddles);

            return paddleData;
        };*/

        //prefabData.unwrap().entities.

        /*world
            .create_entity()
            .with(Paddle::new(Side::Left, 4.0, 16.0))
            .with(left_transform)
            .build();

        world
            .create_entity()
            .with(Paddle::new(Side::Right, 4.0, 16.0))
            .with(right_transform)
            .build();*/
    }
}