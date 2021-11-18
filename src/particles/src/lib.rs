


#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Air = 0,
    Sand = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    material: Material,
    lifetime: f32,
    seed: u8,
    pub last_update_even: bool
}

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Option<Box<Cell>>>>,
    draw_buffer: Vec<u8>,
    update_even: bool
}

impl Cell {
    pub fn new(mat: Material) -> Cell {
        Cell { material: mat, lifetime: 0f32, seed: 0, last_update_even: false }
    }

    fn draw(&self) -> [u8; 4] {
        match self.material {
            Material::Sand => [255, 125, 0, 255],
            _ => [0; 4]
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> World{
        let cells = vec![vec![None; height]; width];

        let draw_buffer = vec![0u8; height*width*4];
        
        World { width, height, cells, draw_buffer, update_even: false }
    }

    pub fn width(&self) -> u32 { self.width as u32 }
    pub fn height(&self) -> u32 { self.height as u32 }
 
    pub fn draw(&mut self) -> &Vec<u8>{
        for y in 0..self.height {
            for x in 0..self.width {
                let data = match &self.cells[x][y] {
                    Some(cell) => cell.draw(),
                    None => [120, 0, 255, 255]
                };
                
                for i in 0..4 {
                    self.draw_buffer[(y * (self.width*4)) + (x*4) + i] = data[i];
                }
            }
        }

        &self.draw_buffer
    }

    pub fn update(&mut self) {
        self.update_even = !self.update_even;
        for y in 0..self.height-1 {
            for x in 0..self.width {
                // let cell = self.cells[x][y].take();
                match self.cells[x][y].take() {
                    Some(mut cell) => {
                        
                        if cell.last_update_even == self.update_even {
                            self.cells[x][y] = Some(cell);
                        } else {
                            cell.last_update_even = !cell.last_update_even;

                            let mut dest = (x, y);

                            if y < self.height-1 {
                                if self.cells[x][y+1].is_none() {
                                    dest = (x, y+1);
                                } else if x > 0 && self.cells[x-1][y+1].is_none() {
                                    dest = (x-1, y+1);
                                } else if x < self.width-1 && self.cells[x+1][y+1].is_none() {
                                    dest = (x+1, y+1);
                                }
                            } 
                                
                            self.cells[dest.0][dest.1] = Some(cell);
                        }

                    },
                    None => ()
                }
            }
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x][y] = Some(Box::new(cell));
    }

}
