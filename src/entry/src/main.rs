use display_gui;
use display_mat;
use particles;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        let world = particles::World::new(64, 32);

        let window = display_gui::Window::new("Sand Test", &world, (15.0, 15.0));
        window.run(world);
    } else {
        display_mat::draw();
    }
}
