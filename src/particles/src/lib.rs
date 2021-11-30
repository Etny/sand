mod cell_list;
mod cell;
mod world_sampler;

use world_sampler::CellContext;
use cell_list::CellList;
use cell::*;

pub use cell::Material;

pub struct World {
    width: u32,
    height: u32,
    cells: CellList,
    clock: u8
}


impl World {
    pub fn new(width: u32, height: u32) -> World{
        let cells = CellList::new(width, height);        
        World { width, height, cells, clock: 0 }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn cells(&self) -> &CellList {
         &self.cells
    }
 
    pub fn draw(&self) -> &Vec<(u8, u8, u8, u8)>{
        self.cells.data()
    }

    pub fn set_cell(&mut self, x: u32, y: u32, material: Material)  {
        self.cells.set_cell(x, y, Cell::new(material, self.clock));
    }

    pub fn update(&mut self) {
        self.clock = self.clock.overflowing_add(1).0;

        for y in (0..self.height).rev() {
            for x in 0..self.width {

                let x = if self.clock % 2 == 0 { x } else { self.width - 1 - x };

                if self.cells.is_empty(x, y) { continue; }
                if self.cells.get_clock(x, y) == self.clock { continue; } 

                let mut cell = self.cells.take_cell(x, y);
                cell.clock = self.clock;

                cell.update(CellContext::new(&mut self.cells, cell, x, y));
            }
        }
    }

}

