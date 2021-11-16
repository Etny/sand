


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
    draw_buffer: Vec<[f32; 4]>,
}

impl Cell {
    fn new() -> Cell {
        Cell { material: Material::Sand, lifetime: 0f32, seed: 0 }
    }

    fn draw(&self) -> [f32; 4] {
        match self.material {
            Material::Air => [1.0, 0.5, 0.5, 1.0],
            Material::Sand => [1.0, 0.4, 1.0, 1.0]
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> World{
        let cells = vec![vec![Cell::new(); height]; width];
        let draw_buffer = vec![[0f32; 4]; height*width];
        
        World { width, height, cells, draw_buffer }
    }

    pub fn draw(&mut self) -> &Vec<[f32; 4]>{
        for x in 0..self.width {
            for y in 0..self.height {
                self.draw_buffer[(x * self.width) + y] = self.cells[x][y].draw();
            }
        }

        &self.draw_buffer
    }

    pub fn print(&self) {
        println!("{:?}", self.cells);
    }
}