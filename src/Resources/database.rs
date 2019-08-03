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

pub trait PrefabHook
{

}

#[macro_export]
macro_rules! register_prefab_hook {
    ($prefab_name: ident, $type_name: ident) => (
        pub fn retrieve_prefab_handle(world: &mut World, path: &str) -> Handle<Prefab<$type_name>>
        {
            let prefab_handle = world.exec(
                |loader: PrefabLoader<'_, $type_name>|
                {
                    loader.load(
                        $prefab_name,
                        RonFormat,
                        ()
                    )
                }
            );

            return prefab_handle;
        }
    )
}



pub struct DataBase;

impl DataBase
{
    /*pub fn retrieve_prefab_handle<T>(world: &mut World, path: &str) -> Handle<Prefab<Paddle>>
    {
        let prefab_handle = world.exec(
            |loader: PrefabLoader<'_, Paddle>|
            {
                loader.load(
                    path,
                    RonFormat,
                    ()
                )
            }
        );

        return prefab_handle;
    }*/
}
