#[macro_use]
extern crate glium;

use particles;

use glium::{HeadlessRenderer, Surface, glutin, texture::buffer_texture::{BufferTexture, BufferTextureType}};
use glutin::dpi::PhysicalSize;


pub struct Renderer {
    render_context: HeadlessRenderer,
    quad_vertex_buffer: glium::VertexBuffer<Vertex>,
    quad_index_buffer: glium::IndexBuffer<u32>,
    shader: glium::Program,
    texture: BufferTexture<(u8, u8, u8, u8)>
}

impl Renderer {
    pub fn new(world: &particles::World) -> Renderer {
        implement_vertex!(Vertex, position, tex_coords);

        let size = PhysicalSize::new(world.width(), world.height());
        let world_size = (world.width() as f32, world.height() as f32);

        let event_loop = glutin::event_loop::EventLoop::new();
        let context = glutin::ContextBuilder::new().build_headless(&event_loop, size).unwrap();
        
        let render_context = HeadlessRenderer::new(context).unwrap();

        let shape = vec![
            Vertex{ position: [-1.0, -1.0 ], tex_coords: [0.0, world_size.1]},
            Vertex{ position: [-1.0, 1.0 ], tex_coords: [0.0, 0.0]},
            Vertex{ position: [1.0, 1.0 ], tex_coords: [world_size.0, 0.0]},
            Vertex{ position: [1.0, -1.0 ], tex_coords: [world_size.0, world_size.1]}
        ];

        let indices: [u32; 6] = [0, 1, 2, 3, 2, 0];

        let quad_vertex_buffer = glium::VertexBuffer::new(&render_context, &shape).unwrap();
        let quad_index_buffer = glium::index::IndexBuffer::new(&render_context, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let vertex_shader_src = include_str!("../vertex.vert");
        let fragment_shader_src = include_str!("../fragment.frag");
        let shader = glium::Program::from_source(&render_context, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        let texture = BufferTexture::persistent(&render_context, &world.cells().data(), BufferTextureType::Unsigned).unwrap();

        Renderer{ 
            render_context,
            quad_index_buffer,
            quad_vertex_buffer,
            shader,
            texture
        }
    }

    pub fn render(&mut self, world: &mut particles::World) -> glium::texture::RawImage2d<u8> {
        let mut target = self.render_context.draw();

        self.texture.write(world.draw());
        let uniforms = uniform! { data: &self.texture, world_size: [world.width(), world.height()] };

        target.clear_color_and_stencil((0.0, 0.0, 0.0, 0.0), 0);
        target.draw(&self.quad_vertex_buffer, &self.quad_index_buffer, &self.shader, &uniforms, &Default::default()).unwrap();
        
        target.finish().unwrap();

        self.render_context.read_front_buffer().unwrap()
    }
}



#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}