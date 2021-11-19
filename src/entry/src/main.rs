use particles;
use display_gui;

fn main() {
    let world = particles::World::new(640, 320);

   let window = display_gui::Window::new("Sand Test", &world, (1.50, 1.50));
   window.run(world);
}
