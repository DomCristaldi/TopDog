use {
  amethyst::{
    assets::{
      PrefabData
    },
    ecs::{
      Entity,
    },
    error::Error,
  },
  serde::{
     Deserialize,
     Serialize,
  },
};

use crate::{
  Resources::Dimensions,
};

/*#[derive(Debug, Deserialize, Serialize, PrefabData)]
pub struct PaddlePrefabData
{
  dimensions: Dimensions,
}*/


/*#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct PaddlePrefab
{

}

impl<'a> PrefabData<'a> for PaddlePrefab
{
    type SystemData = (

    );

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<(), Error>
    {


      //system_data.insert(entity, ).map(|_| ())?;

      return Ok(());
    }
    
}*/
