use particles;
use display_gui;

fn main() {
    let world = particles::World::new(64, 32);

    let window = display_gui::Window::new("Sand Test", world.width() * 15, world.height() * 15);
    window.run(world);
}
