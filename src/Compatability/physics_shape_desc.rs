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


#[derive(Serialize, Deserialize)]
#[serde(remote = "BodyMode")]
enum BodyMode_SerDe
{
    Disabled,
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "RigidBody")]
pub struct RigBodyDesc_PrefabData
{
    /// Body mode
    #[serde(with = "BodyMode_SerDe")]
    pub mode: BodyMode,
    /// Body mass
    pub mass: Option<f32>,
    /// Body friction range 0 - 1
    pub friction: Option<f32>,
    /// Body bounciness range 0 - 1
    pub bounciness: Option<f32>,
    /// Collision Groups this Rigid Body belong.
    /*pub belong_to: Vec<CollisionGroup>,
    /// Collide with groups.
    pub collide_with: Vec<CollisionGroup>,*/
    /// Lock body translation along X
    pub lock_translation_x: Option<bool>,
    /// Lock body translation along Y
    pub lock_translation_y: Option<bool>,
    /// Lock body translation along Z
    pub lock_translation_z: Option<bool>,

    /// Lock body rotation along X
    pub lock_rotation_x: Option<bool>,
    /// Lock body rotation along Y
    pub lock_rotation_y: Option<bool>,
    /// Lock body rotation along Z
    pub lock_rotation_z: Option<bool>,
}

impl Into<RigidBodyDesc<f32>> for RigBodyDesc_PrefabData
{
  fn into(self) -> RigidBodyDesc<f32>
  {
    let mut rb_desc = RigidBodyDesc::default();

    rb_desc.mode = self.mode;

    macro_rules! extract_data{
      ( $x:ident ) =>
      {
        match self.$x
        {
          Some(val) => rb_desc.$x = val,
          _ => (),
        }
      };
    }

    extract_data!(mass);
    extract_data!(bounciness);
    extract_data!(friction);

    extract_data!(lock_translation_x);
    extract_data!(lock_translation_y);
    extract_data!(lock_translation_z);

    extract_data!(lock_rotation_x);
    extract_data!(lock_rotation_y);
    extract_data!(lock_rotation_z);

    //physics_world.rigid_body_server().create(&rb_desc)
    rb_desc
  }
}

/*impl From<RigBodyDesc_PrefabData> for RigidBodyDesc<f32>
{
  pub fn from(rig_bod_prefab_data RigBodyDesc_PrefabData) -> Self
  {

  }
}*/

/*impl RigBodyDesc_PrefabData
{
  pub fn create_component_data(&self) -> RigidBodyDesc<f32>
  {
    let mut rb_desc = RigidBodyDesc::default();

      rb_desc.mode = self.mode;

      macro_rules! extract_data{
        ( $x:ident ) =>
        {
          match self.$x
          {
            Some(val) => rb_desc.$x = val,
            _ => (),
          }
        };
      }

      extract_data!(mass);
      extract_data!(bounciness);
      extract_data!(friction);

      extract_data!(lock_translation_x);
      extract_data!(lock_translation_y);
      extract_data!(lock_translation_z);

      extract_data!(lock_rotation_x);
      extract_data!(lock_rotation_y);
      extract_data!(lock_rotation_z);

      //physics_world.rigid_body_server().create(&rb_desc)
  }
}*/



/*impl<'a> PrefabData<'a> for RigBodyDesc_PrefabData
{
  //type SystemData = WriteStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>;
  type SystemData = (
    WriteExpect<'a, PhysicsWorld<f32>>,
    WriteStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
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
    let (physics_world, rig_bod_handle_storage) = sys_data;

    let rigid_body_comp = {
      let mut rb_desc = RigidBodyDesc::default();

      rb_desc.mode = self.mode;

      macro_rules! extract_data{
        ( $x:ident ) =>
        {
          match self.$x
          {
            Some(val) => rb_desc.$x = val,
            _ => (),
          }
        };
      }

      extract_data!(mass);
      extract_data!(bounciness);
      extract_data!(friction);

      extract_data!(lock_translation_x);
      extract_data!(lock_translation_y);
      extract_data!(lock_translation_z);

      extract_data!(lock_rotation_x);
      extract_data!(lock_rotation_y);
      extract_data!(lock_rotation_z);

      physics_world.rigid_body_server().create(&rb_desc)
    };

    rig_bod_handle_storage.insert(entity, rigid_body_comp).map(|_| ())?;

    Ok(())
  }
}*/

//#[serde(remote = "ShapeDesc")]
//#[derive(Debug, Clone, Serialize, Deserialize, )]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "PhysicalShape")]
pub enum ShapeDesc_PrefabData
{
    Sphere { radius: f32, },
    Cube { half_extents: Vector3<f32>, },
    Capsule { half_height: f32, radius: f32, },
    Cylinder { half_height: f32, radius: f32, },
    Plane,
    //Convex { points: Vec<Point3<f32>>, },
    //TriMesh { points: Vec<Point3<f32>>, indices: Vec<Point3<usize>>, },
    //Compound { shapes: Vec<(Isometry3<f32>, ShapeDesc_PrefabData)>, },
}

impl From<ShapeDesc_PrefabData> for ShapeDesc<f32>
{
  fn from(shape_desc_prefab_data: ShapeDesc_PrefabData) -> ShapeDesc<f32>
  {
    match shape_desc_prefab_data
    {
      ShapeDesc_PrefabData::Sphere{ radius } => ShapeDesc::Sphere{ radius: radius },
      ShapeDesc_PrefabData::Cube{ half_extents } => ShapeDesc::Cube{ half_extents: half_extents },
      ShapeDesc_PrefabData::Capsule{ half_height, radius } => ShapeDesc::Capsule{ half_height, radius },
      ShapeDesc_PrefabData::Cylinder{ half_height, radius, } => ShapeDesc::Cylinder{ half_height: half_height, radius: radius, },
      ShapeDesc_PrefabData::Plane => ShapeDesc::Plane,
      /*ShapeDesc_PrefabData::Convex{ points } => ShapeDesc::Convex{ points: points },
      ShapeDesc_PrefabData::TriMesh{ points, indices } => ShapeDesc::TriMesh{ points, indices },
      ShapeDesc_PrefabData::Compound{ shapes, } =>      physics_world.rigid_body_server().create(&rb_desc)

      {
        let compound_shapes = shapes.into_iter()
          .map( |(isometry, desc_prefab_data)| 
            ( isometry, ShapeDesc::<f32>::from(desc_prefab_data) )
            )
          .collect();
    
          ShapeDesc::Compound{ shapes: compound_shapes }
      },*/
    }
  }
}

// impl<'a> PrefabData<'a> for ShapeDesc_PrefabData
// {
//   type SystemData = (
//     WriteExpect<'a, PhysicsWorld<f32>>,
//     WriteStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
//   );

//   type Result = ();

//   fn add_to_entity(
//     &self,
//     entity: Entity,
//     sys_data: &mut Self::SystemData,
//     entities: &[Entity],
//     children: &[Entity],
//   ) -> Result<(), Error>
//   {
//     //let (physics_world, shape_handle_storage) = sys_data;

//     /*let shape_comp = {
//       //let mut shape_desc = ShapeDesc::default();
//       //let shape_desc: ShapeDesc<f32> = ShapeDesc::<f32>::from( self );
//       let shape_desc = ShapeDesc::<f32>::from( self.clone() );

//       physics_world.shape_server().create(&shape_desc)
//     };

//     shape_handle_storage.insert(entity, shape_comp).map(|_| ())?;*/

//     Ok(())
//   }
// }


// TODO: Bundle all physics prefab crap together into a PhysicsData_Prefab or something like that
//  because we panic when using WriteExpect<'a, PhysicsWorld<f32>> in two different systems

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
//#[serde(rename = "RigidBody")]
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
        let shape_desc = ShapeDesc::<f32>::from( phys_shape.clone() );
        let shape_comp = physics_world.shape_server().create(&shape_desc);

        shape_handle_storage.insert(entity, shape_comp).map(|_| ())?;
      },
      _ => (),
    }



    //let (physics_world, rig_bod_handle_storage, shape_handle_storage) = sys_data;

    /*let shape_comp = {
      //let mut shape_desc = ShapeDesc::default();
      //let shape_desc: ShapeDesc<f32> = ShapeDesc::<f32>::from( self );
      let shape_desc = ShapeDesc::<f32>::from( self.clone() );

      physics_world.shape_server().create(&shape_desc)
    };

    shape_handle_storage.insert(entity, shape_comp).map(|_| ())?;*/

    Ok(())
  }
}