extern crate rand;
extern crate rand_chacha;

use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use num_derive::FromPrimitive;
use super::CellContext;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: Material,
    pub seed: u8,
    pub data: u8,
    pub clock: u8
}

impl Cell {
    pub fn new(material: Material, clock: u8) -> Cell {
        let mut cell = Cell { material, seed: random(), data: 0, clock };
        material.init(&mut cell);
        cell
    }

    pub fn update(self, context: CellContext) {
        self.material.update(self, context);
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Material {
    Air = 0,
    Sand = 1,
    Water = 2,
    Wood = 3,
    Fire = 4,
    Flames = 5,
}

impl Material {
    fn update(&self, cell: Cell, context: CellContext) {
        match *self {
            Material::Sand => update_sand(cell, context),
            Material::Water => update_water(cell, context),
            Material::Fire => update_fire(cell, context),
            Material::Flames => update_flames(cell, context),
            _ => update_none(cell, context)
        }
    }

    fn init(&self, cell: &mut Cell) {
        match *self {
            Material::Water => init_water(cell),
            Material::Fire => init_fire(cell),
            Material::Flames => init_flames(cell),
            _ => ()
        }
    }

    pub fn density(&self) -> f32 {
        match *self {
            Material::Air => 1.0,
            Material::Water => 1.5,
            _ => 10.0
        }
    }

    pub fn flamability(&self) -> f32 {
        match *self {
            Material::Wood => 0.8,
            Material::Water => -1.0,
            _ => 0.0
        }
    }
}

fn update_none(cell: Cell, context: CellContext) {
    context.place(0, 0, cell);
}

fn update_sand(cell: Cell, context: CellContext) {
    let dest = context.test_offsets_density(&[(0, 1), (-1, 1), (1, 1)]);
        
    context.place(dest.0, dest.1, cell);
}


fn update_water(mut cell: Cell, context: CellContext) {
    let mut dest = context.test_offsets_density(&[(0, 1), (-1, 1), (1, 1)]);

    if dest == (0, 0) {
        let flow_dir = if cell.data == 0 { (1, 0) } else { (-1, 0) };

        if context.is_empty(flow_dir.0, flow_dir.1) {
            dest = flow_dir;
        } else {
            cell.data = if cell.data == 0 { 1 } else { 0 };
        }
    }
        
    context.place(dest.0, dest.1, cell);
}

fn update_fire(mut cell: Cell, mut context: CellContext) {

    if cell.data == 0 { 

        let mut rng = rand::thread_rng();

        for pos in offsets_around() {
            let flam = match context.get_material(pos.0, pos.1) {
                None => continue,
                Some(val) if val.flamability() <= 0.0 => continue,
                Some(val) => val.flamability()
            };

            if rng.gen_range(0.0..=1.0) <= flam {
                context.transmute(pos.0, pos.1, Material::Fire);
            }
        }

        return;
    }

    cell.data -= 1;

    if cell.clock % 5 == 0 {
        let dest = context.test_offsets_empty(&[(0, -1), (-1, -1), (1, -1)]);
        
        if dest != (0, 0) {
            context.transmute(dest.0, dest.1, Material::Flames);
        }
    }    

    context.place(0, 0, cell);
}

fn update_flames(mut cell: Cell, context: CellContext) {
    if cell.data == 0 { return; }
    cell.data -= 1;

    let mut rng = ChaCha8Rng::from_seed([cell.seed.overflowing_add(cell.clock).0; 32]);
    let mut dirs = vec![(0, -1), (-1, -1), (1, -1)];
    dirs.rotate_right(rng.gen_range(0..3));

    let dest = context.test_offsets_empty(&dirs);   

    context.place(dest.0, dest.1, cell);
}

fn init_water(cell: &mut Cell) {
    cell.data = cell.seed & 0x01;
}

fn init_fire(cell: &mut Cell) {
    let min_lifetime = 60;
    let max_lifetime = 255;

    let mut rng = rand::thread_rng();
    cell.data = rng.gen_range(min_lifetime..=max_lifetime);
}

fn init_flames(cell: &mut Cell) {
    let min_lifetime = 10;
    let max_lifetime = 30;

    let mut rng = rand::thread_rng();
    cell.data = rng.gen_range(min_lifetime..=max_lifetime);
}

fn offsets_around() -> [(i32, i32); 8] {
    [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]
}

