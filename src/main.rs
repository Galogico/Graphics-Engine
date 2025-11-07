#[macro_use]
extern crate glium;
use std::{collections::HashSet, time::Instant};

use image;
use glium::{winit::{event::ElementState, event_loop::ControlFlow, keyboard::{KeyCode, PhysicalKey}}, Surface}; 
use glutin::{self};
mod shaders;
mod shapes;

fn main() {
    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();




    let image = image::load(std::io::Cursor::new(&include_bytes!("image.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);


    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Spinning Image")
        .build(&event_loop);

    let texture: glium::texture::Texture2d = glium::texture::Texture2d::new(&display, image).unwrap();


    let shape = vec![
        shapes::Vertex { position: [-0.5, -0.5], tex_coords:[0.0, 0.0] },
        shapes::Vertex { position: [ 0.5, -0.5], tex_coords:[1.0, 0.0]},
        shapes::Vertex { position: [ 0.5, 0.5], tex_coords:[1.0, 1.0]},

        shapes::Vertex { position: [ 0.5,  0.5], tex_coords:[1.0, 1.0] },
        shapes::Vertex { position: [ -0.5,  0.5], tex_coords:[0.0, 1.0]},
        shapes::Vertex { position: [ -0.5, -0.5], tex_coords:[0.0, 0.0]}
    ];

    let mut player = shapes::GameObject{
        position: [0.0 ,0.0],
        rotation: 0.0 ,
        scale: [1.0, 1.0],
        texture: &texture,
        shape: shape
    };
    
    let vertex_buffer = player.get_vertex_buffer(&display);


    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&display, shaders::vertex_shader_src, shaders::fragment_shader_src, None).unwrap();

    let mut y: f32 = 0.0;
    let mut x: f32 = 0.0;


    let mut start = Instant::now();

    let mut elapsed = start.elapsed();
    let mut frame_counter = 0;



    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    frame_counter += 1;
                    elapsed = start.elapsed();

                    if elapsed.as_millis() > 1000{
                        println!("{}", frame_counter);
                        frame_counter = 0;
                        start = Instant::now();
                    }

                    if pressed_keys.contains(&KeyCode::KeyW) {
                        y += 0.01;
                    }

                    player.setPosition([x, y]);

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.05, 0.1, 1.0);

                    let uniforms = uniform! {
                        matrix: player.get_matrix(),
                        tex: player.texture
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
    
                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },

                glium::winit::event::WindowEvent::KeyboardInput { event:
                glium::winit::event::KeyEvent {
                physical_key: PhysicalKey::Code(key_code),
                state: ElementState::Pressed,
                ..
                }, 
                 ..
                } => {
                    pressed_keys.insert(key_code);
                }
                

                glium::winit::event::WindowEvent::KeyboardInput { event:
                glium::winit::event::KeyEvent {
                physical_key: PhysicalKey::Code(key_code),
                state: ElementState::Released,
                ..
                }, 
                 ..
                } => {
                    pressed_keys.remove(&key_code);
                },

                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            
            _ => (),
        }
    })
    .unwrap();
}