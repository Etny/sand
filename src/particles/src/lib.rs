


#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Material {
    Air = 0,
    Sand = 1,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    material: Material,
    lifetime: f32,
    seed: u8
}



pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    draw_buffer: Vec<[u8; 4]>,
}

impl Cell {
    fn new(mat: Material) -> Cell {
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
        let mut cells = vec![];

        for i in 0..width {
            let mat = match i%2 {
                0 => Material::Air,
                _ => Material::Sand
            };
            cells.push(vec![Cell::new(mat); height]);
        }

        let draw_buffer = vec![[0u8; 4]; height*width];
        
        World { width, height, cells, draw_buffer }
    }

    pub fn draw(&mut self) -> &Vec<[u8; 4]>{
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_buffer[(y * self.height) + x] = self.cells[x][y].draw();
            }
        }

        &self.draw_buffer
    }

    pub fn print(&self) {
        println!("{:?}", self.cells);
    }
}