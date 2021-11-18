


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
    seed: u8
}



pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    draw_buffer: Vec<u8>,
}

impl Cell {
    pub fn new(mat: Material) -> Cell {
        Cell { material: mat, lifetime: 0f32, seed: 0 }
    }

    fn draw(&self) -> [u8; 4] {
        match self.material {
            Material::Air => [255, 125, 0, 255],
            Material::Sand => [120, 0, 255, 255]
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> World{
        let cells = vec![vec![Cell::new(Material::Sand); height]; width];

        let draw_buffer = vec![0u8; height*width*4];
        
        World { width, height, cells, draw_buffer }
    }

    pub fn width(&self) -> u32 { self.width as u32 }
    pub fn height(&self) -> u32 { self.height as u32 }
 
    pub fn draw(&mut self) -> &Vec<u8>{
        for y in 0..self.height {
            for x in 0..self.width {
                let data = self.cells[x][y].draw();
                
                for i in 0..4 {
                    self.draw_buffer[(y * (self.width*4)) + (x*4) + i] = data[i];
                }
            }
        }

        &self.draw_buffer
    }

    pub fn update(&mut self) {
        
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x][y] = cell;
    }

}