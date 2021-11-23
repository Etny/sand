extern crate rpi_led_matrix;
use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions, LedRuntimeOptions};

pub fn draw() {
    let mut mat_opts = LedMatrixOptions::new();
    let mut run_opts = LedRuntimeOptions::new();

    mat_opts.set_cols(64);
    run_opts.set_gpio_slowdown(1);

    let matrix = LedMatrix::new(Some(mat_opts), Some(run_opts)).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    for red in 0..255 {
        for green in 0..255 {
            for blue in 0..255 {
                canvas.fill(&LedColor { red, green, blue });
                canvas = matrix.swap(canvas);
            }
        }
    }
}

