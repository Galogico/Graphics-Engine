use std::ffi::c_float;

use glium::{glutin::surface::WindowSurface, winit::dpi::Position};

#[derive(Copy, Clone)]
pub struct Vertex {
        pub position: [f32; 2],
        pub tex_coords: [f32; 2],
    }
implement_vertex!(Vertex, position, tex_coords);


pub struct GameObject<'a>{
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
    pub texture: & 'a glium::texture::Texture2d,
    pub shape: Vec<Vertex>
}

impl GameObject <'_>{
    pub fn get_matrix(&self) -> [[f32; 4]; 4]{
        return [
            [self.scale[0], 0.0, 0.0, 0.0],
            [0.0, self.scale[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0]
        ]
    }
    pub fn setPosition(&mut self, position: [f32; 2]){
        self.position = position;
    }

    pub fn setScale(&mut self, scale: [f32; 2]){
        self.scale = scale;
    }

    pub fn get_vertex_buffer(&self, display: &glium::Display<WindowSurface>) -> glium::VertexBuffer<Vertex>{
        return glium::VertexBuffer::new(display, &self.shape).unwrap()
    }
}