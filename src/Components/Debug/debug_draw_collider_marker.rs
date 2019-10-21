use amethyst::{
  ecs::{ Component, NullStorage, WriteStorage},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct DebugDraw_Collider_Marker;

impl Component for DebugDraw_Collider_Marker
{
  type Storage = NullStorage<Self>;
}