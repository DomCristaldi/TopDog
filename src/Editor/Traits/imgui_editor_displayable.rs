pub use amethyst_imgui::{
  imgui,
  imgui::{ im_str, },
};

pub trait ImguiEditorDisplayable<WORLD_CONTEXT>
{
  fn editor_display_mut(&mut self, ui: &imgui::Ui, world: &WORLD_CONTEXT)
  {

  }
  
  /*fn editor_display(&self, ui: &imgui::Ui, world: &WORLD_CONTEXT)
  {

  }*/
}