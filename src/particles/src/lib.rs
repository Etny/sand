use num_derive::FromPrimitive;




#[derive(Clone, Copy, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Material {
    Air = 0,
    Sand = 1,
}

static EMPTY: Cell = Cell { material: Material::Air, seed: 0, data: 0, clock: 0 };

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    material: Material,
    seed: u8,
    data: u8,
    clock: u8
}

pub struct CellList {
    data: Vec<(u8, u8, u8, u8)>,
    width: u32,
    height: u32
}


pub struct World {
    width: u32,
    height: u32,
    cells: CellList,
    clock: u8
}

impl Cell {
    pub fn new(material: Material) -> Cell {
        Cell { material, seed: 0, data: 0, clock: 0 }
    }

    fn draw(&self) -> [u8; 4] {
        match self.material {
            Material::Sand => [255, 125, 0, 255],
            _ => [0; 4]
        }
    }
}

impl CellList {
    fn new(width: u32, height: u32) -> Self{
        CellList{
            data: vec![(0, 0, 0, 0); (width * height * 4) as usize],
            width,
            height
        }
    }

    // fn get_mat(&self, x: u32, y: u32) -> Material {
    //    num::FromPrimitive::from_u8(self.data[self.to_index(x, y)]).unwrap()
    // }

    fn get_cell(&self, x: u32, y: u32) -> Cell {
        let index = self.to_index(x, y);
        let (mat_id, seed, data, clock) = self.data[index];
        let mat = num::FromPrimitive::from_u8(mat_id).unwrap();
        Cell {
            material: mat,
            seed,
            data,
            clock
        }
    }
    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        let index = self.to_index(x, y);
        // self.data[index] = cell.material as u8;
        // self.data[index + 1] = cell.seed;
        // self.data[index + 2] = cell.data;
        // self.data[index + 3] = cell.clock;
        self.data[index] = (cell.material as u8, cell.seed, cell.data, cell.clock);
    }

    pub fn data(&self) -> &Vec<(u8, u8, u8, u8)> {
        &self.data
    }

    fn to_index(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }
}

impl World {
    pub fn new(width: u32, height: u32) -> World{
        let cells = CellList::new(width, height);

      
        
        World { width, height, cells, clock: 0 }
    }

    pub fn width(&self) -> u32 { self.width as u32 }
    pub fn height(&self) -> u32 { self.height as u32 }

    pub fn cells(&mut self) -> &mut CellList {
        &mut self.cells
    }
 
    pub fn draw(&self) -> &Vec<(u8, u8, u8, u8)>{
        self.cells.data()
    }

    pub fn update(&mut self) {
        // self.update_even = !self.update_even;
        // for y in 0..self.height-1 {
        //     for x in 0..self.width {
        //         // let cell = self.cells[x][y].take();
        //         match self.cells[x][y].take() {
        //             Some(mut cell) => {
                        
        //                 if cell.last_update_even == self.update_even {
        //                     self.cells[x][y] = Some(cell);
        //                 } else {
        //                     cell.last_update_even = !cell.last_update_even;

        //                     let mut dest = (x, y);

        //                     if y < self.height-1 {
        //                         if self.cells[x][y+1].is_none() {
        //                             dest = (x, y+1);
        //                         } else if x > 0 && self.cells[x-1][y+1].is_none() {
        //                             dest = (x-1, y+1);
        //                         } else if x < self.width-1 && self.cells[x+1][y+1].is_none() {
        //                             dest = (x+1, y+1);
        //                         }
        //                     } 
                                
        //                     self.cells[dest.0][dest.1] = Some(cell);
        //                 }((y * self.width) + x) as usize

        //             },
        //             None => ()
        //         }
        //     }
        // }
    }

}


