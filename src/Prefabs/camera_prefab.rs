use amethyst::{
  prelude::Builder,
  assets::{
    Handle, Prefab, PrefabData, PrefabLoader, RonFormat, ProgressCounter,
  },
  core::{
    Transform,
  },
  derive::{
    PrefabData,
  },
  ecs::{
    Component, Entity, World, WriteStorage,
  },
  error::Error,
  renderer::camera::{
      Camera, Projection,
    },
};

/*extern crate amethyst_rendy;
use amethyst_rendy::{
  camera::CameraPrefab,
};*/

use serde::{
  Serialize,
  Deserialize,
};

use crate::StaticData::StaticData;




#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraPrefabData
{
  //pub projection: Projection,
  pub left: f32,
  pub right: f32,
  pub bottom: f32,
  pub top: f32,
  pub z_near: f32,
  pub z_far: f32,
}

impl<'a> PrefabData<'a> for CameraPrefabData
{
  type SystemData = WriteStorage<'a, Camera>;

  type Result = ();

  fn add_to_entity(
    &self,
    entity: Entity,
    camera_storage: &mut Self::SystemData,
    entities: &[Entity],
    children: &[Entity]
  ) -> Result<(), Error>
  {
    let projection = Projection::orthographic(
      self.left,
      self.right,
      self.bottom,
      self.top,
      self.z_near,
      self.z_far);

    let camera = Camera::from(projection);
    //let camera = Camera::from(self.projection.clone());

    camera_storage.insert(entity, camera).map(|_| ())?;

    Ok(())
  }
}

impl CameraPrefabData
{
  // TODO: make a macro for this?
  pub fn retrieve_prefab_handle(world: &mut World) -> Handle<Prefab<CameraPrefabData>>
  {
    world.exec( |loader: PrefabLoader<'_, CameraPrefabData>|
    {
      loader.load( "camera.ron", RonFormat, () )
    })
  }

    pub fn initialize(world: &mut World)
  {
    let prefab_handle: Handle<Prefab<CameraPrefabData>> = CameraPrefabData::retrieve_prefab_handle(world);

    let mut transform = Transform::default();
      transform.set_translation_xyz(StaticData::ARENA_WIDTH() * 0.5, StaticData::ARENA_HEIGHT() * 0.5, 1.0);

    world
        .create_entity()
        .with(prefab_handle.clone())
        .with(transform)
        .build();
  }
}