use amethyst::{
  prelude::*,
  ecs::prelude::{
    /* Component */
    Component,
    DenseVecStorage,
    WriteStorage,
  },  
};


pub struct CharacterAttributeData
{
  pub accel: f32,
  pub max_speed: f32,
}

impl CharacterAttributeData
{
  pub fn default() -> CharacterAttributeData
  {
    CharacterAttributeData{
      accel: 1.0,
      max_speed: 50.0,
    }
  }
}

pub enum AttributeCategory
{
  Ground(CharacterAttributeData),
  Air(CharacterAttributeData),
}

pub struct CharacterAttributes
{
  pub attributes: AttributeCategory,
}

impl Component for CharacterAttributes
{
  type Storage = DenseVecStorage<Self>;
}

impl CharacterAttributes
{
  pub fn default() -> CharacterAttributes
  {
    CharacterAttributes
    {
      attributes: AttributeCategory::Ground(CharacterAttributeData::default()),
    }
  }
}



/*impl Default for CharacterAttributes
{
  attributes: T;
}*/