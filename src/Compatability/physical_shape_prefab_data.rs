use amethyst::{
  core::math::{ Vector3, Point3, Isometry3, },
  error::Error,
};

use amethyst_physics::{
  servers::{ ShapeDesc, },
};

use serde::{ Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "PhysicalShape")]
pub enum ShapeDesc_PrefabData
{
    Sphere { radius: f32, },
    Cube { half_extents: Vector3<f32>, },
    Capsule { half_height: f32, radius: f32, },
    Cylinder { half_height: f32, radius: f32, },
    Plane,
    /*Convex { points: Vec<Point3<f32>>, },
    TriMesh { points: Vec<Point3<f32>>, indices: Vec<Point3<usize>>, },
    Compound { shapes: Vec<(Isometry3<f32>, ShapeDesc_PrefabData)>, },*/
}

impl Into<ShapeDesc<f32>> for ShapeDesc_PrefabData
{
  fn into(self) -> ShapeDesc<f32>
  {
    match self
    {
      ShapeDesc_PrefabData::Sphere{ radius } => ShapeDesc::Sphere{ radius: radius },
      ShapeDesc_PrefabData::Cube{ half_extents } => ShapeDesc::Cube{ half_extents: half_extents },
      ShapeDesc_PrefabData::Capsule{ half_height, radius } => ShapeDesc::Capsule{ half_height, radius },
      ShapeDesc_PrefabData::Cylinder{ half_height, radius, } => ShapeDesc::Cylinder{ half_height: half_height, radius: radius, },
      ShapeDesc_PrefabData::Plane => ShapeDesc::Plane,
      /*ShapeDesc_PrefabData::Convex{ points } => ShapeDesc::Convex{ points: points },
      ShapeDesc_PrefabData::TriMesh{ points, indices } => ShapeDesc::TriMesh{ points, indices },
      ShapeDesc_PrefabData::Compound{ shapes, } =>
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

/*impl From<ShapeDesc_PrefabData> for ShapeDesc<f32>
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
      // ShapeDesc_PrefabData::Convex{ points } => ShapeDesc::Convex{ points: points },
      // ShapeDesc_PrefabData::TriMesh{ points, indices } => ShapeDesc::TriMesh{ points, indices },
      // ShapeDesc_PrefabData::Compound{ shapes, } =>
      // {
      //   let compound_shapes = shapes.into_iter()
      //     .map( |(isometry, desc_prefab_data)| 
      //       ( isometry, ShapeDesc::<f32>::from(desc_prefab_data) )
      //       )
      //     .collect();
    
      //     ShapeDesc::Compound{ shapes: compound_shapes }
      // },
    }
  }
}*/