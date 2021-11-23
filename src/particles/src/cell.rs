extern crate rand;

use rand::prelude::*;
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
}

impl Material {
    fn update(&self, cell: Cell, cont4xt: CellContext) {
        match *self {
            Material::Sand => update_sand(cell, cont4xt),
            Material::Water => update_water(cell, cont4xt),
            _ => ()
        }
    }

    fn init(&self, cell: &mut Cell) {
        match *self {
            Material::Water => init_water(cell),
            _ => ()
        }
    }
}

fn update_sand(cell: Cell, context: CellContext) {
    let dest = context.test_offsets(&[(0, 1), (-1, 1), (1, 1)]);
        
    context.place(dest.0, dest.1, cell);
}

fn update_water(mut cell: Cell, context: CellContext) {
    let mut dest = context.test_offsets(&[(0, 1), (-1, 1), (1, 1)]);

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

fn init_water(cell: &mut Cell) {
    cell.data = cell.seed & 0x01;
}
