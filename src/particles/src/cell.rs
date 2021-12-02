extern crate rand;
extern crate rand_chacha;

use std::default;

use super::CellContext;
use num_derive::FromPrimitive;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: Material,
    pub seed: u8,
    pub data: u8,
    pub clock: u8,
}

impl Cell {
    pub fn new(material: Material, clock: u8) -> Cell {
        let mut cell = Cell {
            material,
            seed: random(),
            data: 0,
            clock,
        };
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
    Oil = 6,
    Grass = 7,
}

impl Material {
    fn update(&self, cell: Cell, context: CellContext) {
        match *self {
            Material::Sand => update_sand(cell, context),
            Material::Water => update_water(cell, context),
            Material::Fire => update_fire(cell, context),
            Material::Flames => update_flames(cell, context),
            Material::Oil => update_oil(cell, context),
            Material::Grass => update_grass(cell, context),
            _ => update_none(cell, context),
        }
    }

    fn init(&self, cell: &mut Cell) {
        match *self {
            Material::Water => init_liquid(cell),
            Material::Oil => init_liquid(cell),
            Material::Fire => init_fire(cell),
            Material::Flames => init_flames(cell),
            _ => (),
        }
    }

    pub fn density(&self) -> f32 {
        match *self {
            Material::Fire => 1.0,
            Material::Air => 1.0,
            Material::Water => 1.5,
            Material::Flames => 1.0,
            Material::Oil => 1.2,
            _ => 10.0,
        }
    }

    pub fn flamability(&self) -> f32 {
        match *self {
            Material::Wood => 0.8,
            Material::Oil => 0.95,
            Material::Grass => 0.3,
            _ => 0.0,
        }
    }

    pub fn burn_duration(&self) -> u8 {
        match *self {
            Material::Wood => 120,
            Material::Oil => 100,
            Material::Grass => 120,
            _ => 0,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

fn update_none(cell: Cell, context: CellContext) {
    context.place(0, 0, cell);
}

fn update_sand(cell: Cell, context: CellContext) {
    let dest = context
        .first_offset_any(
            &[(0, 1), (-1, 1), (1, 1)],
            &[context.test_density(cell.material)],
        )
        .unwrap_or((0, 0));

    context.place(dest.0, dest.1, cell);
}

fn update_water(mut cell: Cell, mut context: CellContext) {
    let tests = &[
        context.test_density(cell.material),
        context.test_material(Material::Fire),
        context.test_material(Material::Flames),
    ];
    let mut dest = context.first_offset_any(&[(0, 1), (-1, 1), (1, 1)], tests);

    if let None = dest {
        let flow_dir = if cell.data == 0 { (1, 0) } else { (-1, 0) };
        let flow_dest = context.first_offset_any(&[flow_dir], tests);

        if flow_dest.is_some() {
            dest = flow_dest;
        } else {
            cell.data = if cell.data == 0 { 1 } else { 0 };
        }
    }

    let dest = dest.unwrap_or((0, 0));

    if [Material::Flames, Material::Fire].contains(
        &context
            .get_material(dest.0, dest.1)
            .unwrap_or(Material::Air),
    ) {
        context.transmute(dest.0, dest.1, Cell::new(Material::Air, cell.clock));
    }

    context.place(dest.0, dest.1, cell);
}

fn update_oil(mut cell: Cell, mut context: CellContext) {
    let tests = &[context.test_density(cell.material)];
    let mut dest = context.first_offset_any(&[(0, 1), (-1, 1), (1, 1)], tests);

    if let None = dest {
        let flow_dir = if cell.data == 0 { (1, 0) } else { (-1, 0) };
        let flow_dest = context.first_offset_any(&[flow_dir], tests);

        if flow_dest.is_some() {
            dest = flow_dest;
        } else {
            cell.data = if cell.data == 0 { 1 } else { 0 };
        }
    }

    let dest = dest.unwrap_or((0, 0));

    if !context.is_empty(dest.0, dest.1) {
        if context
            .get_material(dest.0, dest.1)
            .unwrap_or(Material::Air)
            == Material::Fire
        {
            context.transmute(0, 0, Cell::new(Material::Oil, cell.clock)); // To ensure the spawn_fire call doesn't read air
            spawn_fire(&mut context, 0, 0, cell.clock);
            return;
        } else {
            context.transmute(dest.0, dest.1, Cell::new(Material::Air, cell.clock));
        }
    }

    context.place(dest.0, dest.1, cell);
}

fn update_fire(mut cell: Cell, mut context: CellContext) {
    if cell.data == 0 {
        for pos in offsets_around() {
            try_burn(&mut context, pos.0, pos.1, cell.clock);
        }
        return;
    }

    cell.data -= 1;

    if cell.data % 15 == 0 {
        let dest = context.first_offset_any(&[(0, -1), (-1, -1), (1, -1)], &[context.test_empty()]);

        if let Some(pos) = dest {
            context.transmute(pos.0, pos.1, Cell::new(Material::Flames, cell.clock));
        }
    }

    if cell.data % 5 == 0 {
        cell.seed = random();
    }

    context.place(0, 0, cell);
}

fn update_flames(mut cell: Cell, mut context: CellContext) {
    if cell.data == 0 {
        return;
    }
    cell.data -= 1;

    let mut rng = ChaCha8Rng::from_seed([cell.seed.overflowing_add(cell.clock).0; 32]);
    let mut dirs = vec![(0, -1), (-1, -1), (1, -1)];
    dirs.rotate_right(rng.gen_range(0..3));

    let dest = context.first_offset_any(&dirs, &[context.test_empty()]);

    if let Some(pos) = dest {
        context.place(pos.0, pos.1, cell);
    } else {
        for dir in dirs {
            try_burn(&mut context, dir.0, dir.1, cell.clock);
        }
    }
}

fn update_grass(mut cell: Cell, mut context: CellContext) {
    if context.get_material(0, -1).unwrap_or_default() == Material::Water {
        context.transmute(0, -1, Cell::new(Material::Air, 0));

        let mut rng = rand::thread_rng();
        cell.data += rng.gen_range(1..5);

        if cell.data >= 100 {
            cell.material = Material::Sand;
        }
    }

    context.place(0, 0, cell);
}

fn init_liquid(cell: &mut Cell) {
    cell.data = cell.seed & 0x01;
}

fn try_burn(context: &mut CellContext, dx: i32, dy: i32, clock: u8) {
    let flam = match context.get_material(dx, dy) {
        None => return,
        Some(val) if val.flamability() <= 0.0 => return,
        Some(val) => val.flamability(),
    };

    let mut rng = rand::thread_rng();

    if rng.gen_range(0.0..=1.0) <= flam {
        spawn_fire(context, dx, dy, clock);
    }
}

fn spawn_fire(context: &mut CellContext, dx: i32, dy: i32, clock: u8) {
    let mat = context.get_material(dx, dy).unwrap_or(Material::Air);

    if mat.burn_duration() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let burn_duration: u8 = rng.gen_range(mat.burn_duration()..mat.burn_duration() * 2);

    let mut cell = Cell::new(Material::Fire, clock);
    cell.data = burn_duration;

    context.transmute(dx, dy, cell);
}

fn init_fire(cell: &mut Cell) {
    let min_lifetime = 60;
    let max_lifetime = 255;

    let mut rng = rand::thread_rng();
    cell.data = rng.gen_range(min_lifetime..=max_lifetime);
}

fn init_flames(cell: &mut Cell) {
    let min_lifetime = 3;
    let max_lifetime = 15;

    let mut rng = rand::thread_rng();
    cell.data = rng.gen_range(min_lifetime..=max_lifetime);
}

fn offsets_around() -> [(i32, i32); 8] {
    [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ]
}
