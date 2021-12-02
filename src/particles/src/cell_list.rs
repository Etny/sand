use super::*;

pub struct CellList {
    data: Vec<(u8, u8, u8, u8)>,
    width: u32,
    height: u32,
}

impl CellList {
    pub fn new(width: u32, height: u32) -> Self {
        CellList {
            data: vec![(0, 0, 0, 0); (width * height * 4) as usize],
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Cell {
        let index = self.to_index(x, y);
        let (mat_id, seed, data, clock) = self.data[index];
        let mat = num::FromPrimitive::from_u8(mat_id).unwrap();
        Cell {
            material: mat,
            seed,
            data,
            clock,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn take_cell(&mut self, x: u32, y: u32) -> Cell {
        let cell = self.get_cell(x, y);
        self.clear_cell(x, y);
        cell
    }

    pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.data[self.to_index(x, y)].0 == 0
    }

    pub fn get_material(&self, x: u32, y: u32) -> Material {
        num::FromPrimitive::from_u8(self.data[self.to_index(x, y)].0).unwrap()
    }

    pub fn get_clock(&self, x: u32, y: u32) -> u8 {
        self.data[self.to_index(x, y)].3
    }

    pub fn clear_cell(&mut self, x: u32, y: u32) {
        let index = self.to_index(x, y);
        self.data[index] = (0, 0, 0, 0);
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        let index = self.to_index(x, y);
        self.data[index] = (cell.material as u8, cell.seed, cell.data, cell.clock);
    }

    pub fn data(&self) -> &Vec<(u8, u8, u8, u8)> {
        &self.data
    }

    pub fn to_index(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }
}
