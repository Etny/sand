#[macro_use]
extern crate glium;
extern crate image;

use particles;


use std::time;

use glium::{Display, Surface, glutin};
use glutin::dpi::{PhysicalSize, Size};

pub fn open_window(title: &str) {
    implement_vertex!(Vertex, position, color);
    
    let size = PhysicalSize::new(255, 255);

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title(title).with_inner_size(Size::Physical(size));
    let context_builder = glutin::ContextBuilder::new();

    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    let mut world = particles::World::new(5, 5);
    let draw = world.draw();
    let mut shape = vec![Vertex{position:[0u32; 2], color:[0f32; 4]}; 25];

    for x in 0..5 {
        for y in 0..5 {
            shape[y + (x * 5)] = Vertex { position: [x as u32, y as u32], color: draw[(x * 5) + y] };
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = include_str!("../vertex.vert");

    let fragment_shader_src = include_str!("../fragment.frag");

    let shader = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let frame_time = 16_666_667;
    let point_size = (size.width / 5) as f32;

    event_loop.run(move |event, _, control_flow| {

        let mut target = display.draw();
        // target.clear_color(1.0, 0.5, 0.3, 1.0);

        let uniforms = uniform! { dimensions: [5u32, 5u32], point_size: point_size };

        let params = glium::DrawParameters {
            point_size: Some(point_size),
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
    position: [u32; 2],
    color: [f32; 4]
}
