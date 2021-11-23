extern crate rpi_led_matrix;

use std::thread;

use rpi_led_matrix::{LedMatrix, LedColor};

pub fn draw() {
    let matrix = LedMatrix::new(None, None).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    for red in 0..255 {
        for green in 0..255 {
            for blue in 0..255 {
                canvas.fill(&LedColor { red, green, blue });
                canvas = matrix.swap(canvas);
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}

