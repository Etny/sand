#[macro_use]
extern crate glium;

use particles;
use std::time;
use glium::{Display, Surface, glutin, texture::buffer_texture::{BufferTexture, BufferTextureType}};
use glutin::dpi::{PhysicalSize, Size};


pub struct Window {
    display: Display,
    event_loop: Option<glutin::event_loop::EventLoop<()>>,
    shader: glium::Program,
    quad_vertex_buffer: glium::VertexBuffer<Vertex>,
    quad_index_buffer: glium::IndexBuffer<u32>,
    mouse_pos: Option<(f64, f64)>,
    size: (u32, u32),
    cell_size: (f32, f32),
    mouse_down: bool,
    selected_material: particles::Material
}

impl Window {
    pub fn new(title: &str, world: &particles::World, cell_size: (f32, f32)) -> Self {
        implement_vertex!(Vertex, position, tex_coords);
    
        let world_size = (world.width() as f32, world.height() as f32);
        let size = PhysicalSize::new((world.width() as f32 * cell_size.0).floor() as u32, (world.height() as f32 * cell_size.1).floor() as u32);

        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new().with_title(title).with_inner_size(Size::Physical(size));
        let context_builder = glutin::ContextBuilder::new();

        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        let shape = vec![
            Vertex{ position: [-1.0, -1.0 ], tex_coords: [0.0, world_size.1]},
            Vertex{ position: [-1.0, 1.0 ], tex_coords: [0.0, 0.0]},
            Vertex{ position: [1.0, 1.0 ], tex_coords: [world_size.0, 0.0]},
            Vertex{ position: [1.0, -1.0 ], tex_coords: [world_size.0, world_size.1]}
        ];

        let indices: [u32; 6] = [0, 1, 2, 3, 2, 0];

        let quad_vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let quad_index_buffer = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let vertex_shader_src = include_str!("../vertex.vert");

        let fragment_shader_src = include_str!("../fragment.frag");

        let shader = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        Window { 
            display, 
            event_loop: Some(event_loop), 
            shader, 
            quad_vertex_buffer, 
            quad_index_buffer, 
            mouse_pos: None, 
            mouse_down: false,
            size:(size.width, size.height),
            cell_size,
            selected_material: particles::Material::Sand
        }
    }

    pub fn mouse_in_window(&self) -> bool {
        match self.mouse_pos {
            Some(pos) => {
                pos.0 < self.size.0 as f64 &&  pos.1 < self.size.1 as f64
            },
            None => false
        }
    }

    pub fn select_material(&mut self, material: particles::Material) {
        self.selected_material = material;
        println!("Selected {:?}", material);
    }

    pub fn run(mut self, mut world: particles::World) {
        let event_loop = self.event_loop.take().unwrap();
        let tick_time = time::Duration::from_nanos(16_666_667);
        let mut last_tick = time::Instant::now();

        // let mut last_time = time::Instant::now();
        // let mut count: u32 = 0;

        let texture = BufferTexture::persistent(&self.display, &world.cells().data(), BufferTextureType::Unsigned).unwrap();

        event_loop.run(move |event, _, control_flow| {
            let now = time::Instant::now();
            
            match event {
                glutin::event::Event::WindowEvent { ref event, ..} => match event {
    
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },

                    glutin::event::WindowEvent::Resized(size) => {
                        self.size = (size.width, size.height);
                        self.cell_size = (size.width as f32 / world.width() as f32, size.height as f32 / world.height() as f32);
                    },
    
                    glutin::event::WindowEvent::CursorMoved {position, ..} => {
                        self.mouse_pos = Some((position.x, position.y));
                    },
    
                    glutin::event::WindowEvent::MouseInput {state, button, ..} => {
                        if *button == glutin::event::MouseButton::Left {
                            self.mouse_down = *state == glutin::event::ElementState::Pressed;
                        }
                    },

                    glutin::event::WindowEvent::KeyboardInput {input, ..} => {
                        if input.state == glutin::event::ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                glutin::event::VirtualKeyCode::Key1 => self.select_material(particles::Material::Sand),
                                glutin::event::VirtualKeyCode::Key2 => self.select_material(particles::Material::Water),
                                _ => ()
                            }
                        }
                    },
    
                    _ => ()
                },
    
                glutin::event::Event::MainEventsCleared => {
    
                    if now - last_tick >= tick_time {
                        last_tick = now.clone();
                        
                        // count += 1;
                        // if now - last_time >= time::Duration::from_secs(1) {
                        //     println!("{} fps", count);
                        //     count = 0;
                        //     last_time = time::Instant::now();
                        // }

                        world.update();
                        
                        if self.mouse_down && self.mouse_in_window() {
                            if let Some(pos) = self.mouse_pos {
                               world.set_cell((pos.0 as f32 / self.cell_size.0) as u32, (pos.1 as f32 / self.cell_size.1) as u32, self.selected_material);
                            }
                        }
        
                        self.draw(&mut world, &texture);

                        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(now + tick_time);
                    }
                },
    
                _ => ()
            }

            
        });
    }

    fn draw(&self, world: &mut particles::World, texture: &BufferTexture<(u8, u8, u8, u8)>) {
        texture.write(world.draw());

        let uniforms = uniform! { data: texture, world_size: [world.width(), world.height()] };

        let mut target = self.display.draw();

        target.draw(&self.quad_vertex_buffer, &self.quad_index_buffer, &self.shader, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    }
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
