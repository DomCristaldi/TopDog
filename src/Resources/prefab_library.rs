use amethyst::{
  assets::{
    Handle,
    Prefab,
    PrefabData,
    PrefabLoader,
    RonFormat,
    ProgressCounter,
  },
  derive::{
    PrefabData,
  },
  ecs::{
    Entity,
    World,
  },
  error::Error,
};

use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  Prefabs::*,
};

#[derive(Debug, Clone, PrefabData)]
pub enum PrefabType
{
  Paddle,
  Camera,
  CollidableSurface,

  // Paddle(Prefabs::PaddlePrefab),
  // Camera(Prefabs::CameraPrefabData),
  // CollidableSurface(Prefabs::CollidableSurfacePrefab),

  // Paddle("paddle.ron"),
  // Camera("camera.ron"),
  // CollidableSurface("collidable_surface.ron"),
}

impl PrefabType
{
  fn get_prefab_path(&self) -> Result<String, ()>
  {
    match self
    {
      PrefabType::Paddle => Ok("paddle.ron".to_string()),
      PrefabType::Camera => Ok("camera.ron".to_string()),
      PrefabType::CollidableSurface => Ok("collidable_surface.ron".to_string()),
      _ => Err(()),
    }
  }


  // pub fn retrieve_prefab_handle<'a, T>(&self, world: &mut World) -> Result<Handle<Prefab<T>>, ()>
  //   where
  //     T: Send + Sync + 'static + PrefabData<'a>,
  // {
  //   match self.get_prefab_path()
  //   {
  //     Ok(path) =>
  //     {
  //       world.exec( |loader: PrefabLoader<'_, T>|
  //       {
  //         Ok(loader.load( path, RonFormat, ()))
  //       })
  //       //Err(())
  //     },
  //     _ => Err(()),
  //   }
  // }
}


/*impl PrefabType
{
  pub fn retrieve_prefab()
}*/

/*
pub fn retrieve_prefab_handle<'a, T: std::marker::Sync + std::marker::Sync> (prefab_path: &str, world: &mut World) -> &'a Handle<Prefab<T>>
    {
      let prefab_handle = world.exec(
        |loader: PrefabLoader<'_, T>|
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
    */