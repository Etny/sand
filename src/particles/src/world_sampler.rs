
use super::*;

pub struct CellContext<'a>{
    cells: &'a mut CellList,
    x: u32,
    y: u32
}

impl<'a> CellContext<'a> {
    pub fn new(cells: &'a mut CellList, x: u32, y: u32) -> Self {
        Self { cells, x, y }
    }

    pub fn is_empty(&self, dx: i32, dy: i32) -> bool {
        let pos = (self.x as i32 + dx, self.y as i32 + dy);
        if pos.0 < 0 
        || pos.0 as u32 >= self.cells.width() 
        || pos.1 < 0
        || pos.1 as u32 >= self.cells.height() 
        { return false; }
        
        self.cells.is_empty(pos.0 as u32, pos.1 as u32)
    }

    pub fn test_offsets(&self, offsets: &[(i32, i32)]) -> (i32, i32) {
        let mut dest = (0, 0);

        for offset in offsets {
            if self.is_empty(offset.0, offset.1) {
                dest = *offset;
                break;
            }
        }
        
        dest
    }



    pub fn place(self, dx: i32, dy: i32, cell: Cell) {
        let pos = (self.x as i32 + dx, self.y as i32 + dy);
        if pos.0 < 0 
        || pos.0 as u32 >= self.cells.width() 
        || pos.1 < 0
        || pos.1 as u32 >= self.cells.height() 
        { panic!("Cell moved off grid"); }
        
        self.cells.set_cell(pos.0 as u32, pos.1 as u32, cell);
    }

}