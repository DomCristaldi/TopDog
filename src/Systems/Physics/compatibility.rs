
use amethyst::core::Transform;
use specs_physics::{
  bodies::Position,
  nalgebra::Isometry3,
};

impl Position<f32> for Transform {
    fn isometry(&self) -> &Isometry3<f32> {
        self.isometry()
    }

    fn isometry_mut(&mut self) -> &mut Isometry3<f32> {
        self.isometry_mut()
    }

    fn set_isometry(&mut self, isometry: &Isometry3<f32>) -> &mut Self {
        self.set_isometry(*isometry)
    }
}