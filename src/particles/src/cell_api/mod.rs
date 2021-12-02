mod cell_test;

use super::*;
use cell_test::*;

pub struct CellContext<'a> {
    cells: &'a mut CellList,
    x: u32,
    y: u32,
}

impl<'a> CellContext<'a> {
    pub fn new(cells: &'a mut CellList, x: u32, y: u32) -> Self {
        Self { cells, x, y }
    }

    pub fn get_material(&self, dx: i32, dy: i32) -> Option<Material> {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return None,
        };

        Some(self.cells.get_material(pos.0, pos.1))
    }

    pub fn get_clock(&self, dx: i32, dy: i32) -> Option<u8> {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return None,
        };

        Some(self.cells.get_clock(pos.0, pos.1))
    }

    pub fn is_empty(&self, dx: i32, dy: i32) -> bool {
        self.test_offset(dx, dy, self.test_empty())
    }

    pub fn test_offset(&self, dx: i32, dy: i32, test: Box<dyn CellTest>) -> bool {
        test.test(self, dx, dy)
    }

    pub fn test_material(&self, material: Material) -> Box<dyn CellTest> {
        Box::new(MaterialTest::new(material))
    }

    pub fn test_empty(&self) -> Box<dyn CellTest> {
        Box::new(MaterialTest::new(Material::Air))
    }

    pub fn test_density(&self, material: Material) -> Box<dyn CellTest> {
        Box::new(DensityTest::new(material.density()))
    }

    pub fn place(self, dx: i32, dy: i32, cell: Cell) {
        let pos = self.to_real_pos(dx, dy).expect("Cell moved off grid");

        if !self.test_offset(dx, dy, self.test_empty()) {
            if dy < 0 {
                panic!(
                    "Can only move other cells up, {:?} tried to move {}, {} into {:?}",
                    cell,
                    dx,
                    dy,
                    self.get_material(dx, dy)
                )
            }

            let temp = self.cells.take_cell(pos.0 as u32, pos.1 as u32);

            if dy > 0 {
                let dest = self
                    .first_offset_any(&[(-1, 0), (1, 0), (0, 0)], &[self.test_empty()])
                    .unwrap();
                let dest = self
                    .to_real_pos(dest.0, dest.1)
                    .expect("Bumped cell failed to find spot");
                self.cells.set_cell(dest.0, dest.1, temp);
            } else {
                let dest = self.to_real_pos(0, 0).unwrap();
                self.cells.set_cell(dest.0, dest.1, temp);
            }
        }

        self.cells.set_cell(pos.0, pos.1, cell);
    }

    pub fn transmute(&mut self, dx: i32, dy: i32, cell: Cell) {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return,
        };

        self.cells.set_cell(pos.0, pos.1, cell);
    }

    pub fn first_offset_any(
        &self,
        offsets: &[(i32, i32)],
        tests: &[Box<dyn CellTest>],
    ) -> Option<(i32, i32)> {
        let mut result = None;

        for offset in offsets {
            if tests.into_iter().any(|t| t.test(self, offset.0, offset.1)) {
                result = Some(*offset);
                break;
            }
        }

        result
    }
}

impl<'a> CellContext<'a> {
    fn to_real_pos(&self, dx: i32, dy: i32) -> Option<(u32, u32)> {
        let pos = (self.x as i32 + dx, self.y as i32 + dy);
        if pos.0 < 0
            || pos.0 as u32 >= self.cells.width()
            || pos.1 < 0
            || pos.1 as u32 >= self.cells.height()
        {
            return None;
        };
        Some((pos.0 as u32, pos.1 as u32))
    }
}
