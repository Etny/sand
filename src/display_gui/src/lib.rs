#[macro_use]
extern crate glium;
extern crate image;

use particles;


use std::time;

use glium::{Display, Surface, glutin, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}};
use glutin::dpi::{PhysicalSize, Size};

pub fn open_window(title: &str) {
    implement_vertex!(Vertex, position, tex_coords);
    
    let size = PhysicalSize::new(255, 255);

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

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let vertex_shader_src = include_str!("../vertex.vert");

    let fragment_shader_src = include_str!("../fragment.frag");

    let shader = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let frame_time = 16_666_667;

    
    let mut world = particles::World::new(5, 5);
    let draw = world.draw().iter().flatten().map(|x|*x).collect();
    let image = glium::texture::RawImage2d::from_raw_rgba(draw, (5, 5));
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    
    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    event_loop.run(move |event, _, control_flow| {

        let mut target = display.draw();
        // target.clear_color(1.0, 0.5, 0.3, 1.0);

        let uniforms = uniform! { data: glium::uniforms::Sampler(&texture, behavior) };

        let params = glium::DrawParameters {
            // point_size: Some(point_size),
            ..Default::default()
        };

        target.draw(&vertex_buffer, &index_buffer, &shader, &uniforms, &params).unwrap();
        target.finish().unwrap();

        let next_frame = time::Instant::now() + time::Duration::from_nanos(frame_time);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame);

        match event {
            glutin::event::Event::WindowEvent { event, ..} => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return
            },
            _ => ()
        }
    });
    
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
