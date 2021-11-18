#[macro_use]
extern crate glium;
extern crate image;

use particles;


use std::time;

use glium::{Display, Surface, glutin, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}};
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
    mouse_down: bool
}

impl Window {
    pub fn new(title: &str, world: &particles::World, cell_size: (f32, f32)) -> Self {
        implement_vertex!(Vertex, position, tex_coords);
    
        let size = PhysicalSize::new((world.width() as f32 * cell_size.0).floor() as u32, (world.height() as f32 * cell_size.1).floor() as u32);

        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new().with_title(title).with_inner_size(Size::Physical(size));
        let context_builder = glutin::ContextBuilder::new();

        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        let shape = vec![
            Vertex{ position: [-1.0, -1.0 ], tex_coords: [0.0, 0.0]},
            Vertex{ position: [-1.0, 1.0 ], tex_coords: [0.0, 1.0]},
            Vertex{ position: [1.0, 1.0 ], tex_coords: [1.0, 1.0]},
            Vertex{ position: [1.0, -1.0 ], tex_coords: [1.0, 0.0]}
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
            cell_size
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

    pub fn run(mut self, mut world: particles::World) {
        let event_loop = self.event_loop.take().unwrap();

        let mut last_time = time::Instant::now();
        let mut count: u32 = 0;

        let tick_time = time::Duration::from_nanos(16_666_667);
        let mut last_tick = time::Instant::now();
        
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
    
                    _ => ()
                },
    
                glutin::event::Event::MainEventsCleared => {
    
                    if now - last_tick >= tick_time {
                        count += 1;
                        if now - last_time >= time::Duration::from_secs(1) {
                            println!("{} fps", count);
                            count = 0;
                            last_time = time::Instant::now();
                        }

                        if self.mouse_down && self.mouse_in_window() {
                            if let Some(pos) = self.mouse_pos {
                                world.set_cell((pos.0 as f32 / self.cell_size.0) as usize, (pos.1 as f32 / self.cell_size.1)as usize, particles::Cell::new(particles::Material::Sand));
                            }
                        }
        
                        world.update();
        
                        self.draw(&mut world);

                        last_tick = now.clone();
                        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(now + tick_time);
                    }
                },
    
                _ => ()
            }

            
        });
    }

    fn draw(&self, world: &mut particles::World) {
        let size = (world.width(), world.height());
        let draw = world.draw();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(draw, size);
        let texture = glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();
        
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        let mut target = self.display.draw();
        let uniforms = uniform! { data: glium::uniforms::Sampler(&texture, behavior) };

        target.draw(&self.quad_vertex_buffer, &self.quad_index_buffer, &self.shader, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    }
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
