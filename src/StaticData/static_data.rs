

pub struct StaticData
{
  /*pub ARENA_HEIGHT: f32 = 100.0,
  pub ARENA_WIDTH: f32 = 100.0,*/
}

impl StaticData
{
  #[inline]
  pub fn ARENA_WIDTH() -> f32 { 100.0 }
  
  #[inline]
  pub fn ARENA_HEIGHT() -> f32 { 100.0 }
}