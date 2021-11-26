extern crate rpi_led_matrix;
extern crate signal_hook;

use std::sync::{Arc, atomic::{Ordering, AtomicBool}};
use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions, LedRuntimeOptions};

pub fn draw() {
    let interupt_received = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interupt_received)).unwrap();

    let mut mat_opts = LedMatrixOptions::new();
    let mut run_opts = LedRuntimeOptions::new();

    mat_opts.set_cols(64);
    mat_opts.set_chain_length(1);
    mat_opts.set_parallel(1);
    mat_opts.set_hardware_pulsing(true);
    mat_opts.set_pwm_lsb_nanoseconds(700);
    run_opts.set_gpio_slowdown(2);

    let matrix = LedMatrix::new(Some(mat_opts), Some(run_opts)).unwrap();
    
    while !interupt_received.load(Ordering::Relaxed) {
        let mut canvas = matrix.offscreen_canvas();
        for x in 0..64 {
            for y in 0..32 {
                canvas.set(x, y, & LedColor{ red: (x * 4) as u8, green: (y * 4) as u8, blue: 0 });
            }
        }
        matrix.swap(canvas);
    }

    matrix.canvas().clear();
    println!("Received Ctrl+C");
}

