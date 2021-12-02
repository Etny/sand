use super::*;

pub trait CellTest {
    fn test(&self, context: &CellContext, dx: i32, dy: i32) -> bool;
}

pub struct MaterialTest {
    material: Material,
}

pub struct DensityTest {
    density: f32,
}

impl MaterialTest {
    pub fn new(material: Material) -> Self {
        Self { material }
    }
}

impl DensityTest {
    pub fn new(density: f32) -> Self {
        Self { density }
    }
}

impl CellTest for MaterialTest {
    fn test(&self, context: &CellContext, dx: i32, dy: i32) -> bool {
        let pos = match context.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return false,
        };

        context.cells.get_material(pos.0, pos.1) == self.material
    }
}

impl CellTest for DensityTest {
    fn test(&self, context: &CellContext, dx: i32, dy: i32) -> bool {
        let pos = match context.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return false,
        };

        context.cells.get_material(pos.0, pos.1).density() < self.density
    }
}
