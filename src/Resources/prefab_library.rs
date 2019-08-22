use {
  amethyst::{
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
  },
  serde::{
     Deserialize,
     Serialize,
  },
};

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