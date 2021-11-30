
use super::*;

pub struct CellContext<'a>{
    cells: &'a mut CellList,
    cell: Cell,
    x: u32,
    y: u32,
}

impl<'a> CellContext<'a> {
    pub fn new(cells: &'a mut CellList, cell: Cell, x: u32, y: u32) -> Self {
        Self { cells, cell, x, y }
    }

    pub fn get_material(&self, dx: i32, dy: i32) -> Option<Material> {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return None 
        };

        Some(self.cells.get_material(pos.0, pos.1))
    }
    
    pub fn is_empty(&self, dx: i32, dy: i32) -> bool {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return false
        };
        
        self.cells.is_empty(pos.0 as u32, pos.1 as u32)
    }

    pub fn is_lower_density(&self, dx: i32, dy: i32) -> bool {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return false
        };
        
        self.cells.get_material(pos.0 as u32, pos.1 as u32).density() < self.cell.material.density()
    }

    pub fn test_offsets_density(&self, offsets: &[(i32, i32)]) -> (i32, i32) {
        self.test_offsets(offsets, Self::is_lower_density)
    }

    pub fn test_offsets_empty(&self, offsets: &[(i32, i32)]) -> (i32, i32) {
        self.test_offsets(offsets, Self::is_empty)
    }

    pub fn place(self, dx: i32, dy: i32, cell: Cell) {
        let pos = self.to_real_pos(dx, dy).expect("Cell moved off grid");

        if !self.is_empty(dx, dy) {
            if dy <= 0 { panic!("Can only move other cells up") }
            let temp = self.cells.take_cell(pos.0 as u32, pos.1 as u32);

            let dest = self.test_offsets(&[(-1, 0), (1, 0), (0, 0)], Self::is_empty);

            let dest = self.to_real_pos(dest.0, dest.1).expect("Bumped cell failed to find spot");

            self.cells.set_cell(dest.0, dest.1, temp);
        }

        self.cells.set_cell(pos.0, pos.1, cell);
    }

    pub fn transmute(&mut self, dx: i32, dy: i32, material: Material) {
        let pos = match self.to_real_pos(dx, dy) {
            Some(val) => val,
            None => return
        };

        self.cells.set_cell(pos.0, pos.1, Cell::new(material, self.cell.clock));
    }
}

impl<'a> CellContext<'a> { 
    fn test_offsets<F>(&self, offsets: &[(i32, i32)], test: F) -> (i32, i32)
    where F: Fn(&Self, i32, i32) -> bool {
        let mut dest = (0, 0);

        for offset in offsets {
            if test(self, offset.0, offset.1) {
                dest = *offset;
                break;
            }
        }
        
        dest
    }

    fn to_real_pos(&self, dx: i32, dy: i32) -> Option<(u32, u32)> {
        let pos = (self.x as i32 + dx, self.y as i32 + dy);
        if pos.0 < 0 
        || pos.0 as u32 >= self.cells.width() 
        || pos.1 < 0
        || pos.1 as u32 >= self.cells.height() 
        { return None };
        Some((pos.0 as u32, pos.1 as u32))
    }

  
}