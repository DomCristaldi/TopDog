use amethyst::{
  ecs::{ Entity, SystemData, Write, WriteExpect, WriteStorage, },
  core::math::{ Vector3, Point3, Isometry3, },
  assets::{
      Handle,
      Prefab,
      PrefabData,
      PrefabLoader,
      RonFormat,
      ProgressCounter,
    },
  derive::{ PrefabData, },
  error::Error,
};

use amethyst_physics::{
  objects::{ CollisionGroup, PhysicsHandle, PhysicsRigidBodyTag, PhysicsShapeTag, },
  servers::{ PhysicsWorld, BodyMode, RigidBodyDesc, ShapeDesc, },
};

use serde::{ Serialize, Deserialize, Serializer, ser::SerializeStruct, Deserializer, de::Visitor };

use super::{
  rigid_body_prefab_data::RigBodyDesc_PrefabData,
  physical_shape_prefab_data::ShapeDesc_PrefabData,
};


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicsalDesc
{
  rigidbody: Option<RigBodyDesc_PrefabData>,
  physicalShape: Option<ShapeDesc_PrefabData>,
}

impl<'a> PrefabData<'a> for PhysicsalDesc
{
  type SystemData = (
    WriteExpect<'a, PhysicsWorld<f32>>,
    WriteStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
    WriteStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
  );

  type Result = ();

  fn add_to_entity(
    &self,
    entity: Entity,
    sys_data: &mut Self::SystemData,
    entities: &[Entity],
    children: &[Entity],
  ) -> Result<(), Error>
  {

    let (physics_world, rig_bod_handle_storage, shape_handle_storage) = sys_data;

    match self.rigidbody
    {
      Some(rigBod) =>
      {
        let rigid_body_comp = physics_world.rigid_body_server().create(&rigBod.into());
        rig_bod_handle_storage.insert(entity, rigid_body_comp).map(|_| ())?;
      },
      _ => (),
    }

    match self.physicalShape
    {
      Some(phys_shape) =>
      {
        let shape_comp = physics_world.shape_server().create(&phys_shape.into());
        shape_handle_storage.insert(entity, shape_comp).map(|_| ())?;
      },
      _ => (),
    }

    Ok(())
  }
}