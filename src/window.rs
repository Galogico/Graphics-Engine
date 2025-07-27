
use std::{collections::btree_map::Values, usize};

use minifb;

pub struct Window{
    window: minifb::Window,
    framebuffer: FrameBuffer
}
pub struct FrameBuffer{
    data: Vec<u32>,
    width: usize,
    height: usize
}

impl Window {
        pub fn new (name: &str, width: usize, height: usize) -> Self{
            let options = minifb::WindowOptions {
                resize: false,
                ..Default::default()
            };
            let window: minifb::Window = minifb::Window::new(name, width, height, options).expect("Failed to create window");
    
            Window {
                window,
                framebuffer: FrameBuffer::new(width, height)
            }
        }
        pub fn frame_buffer(&mut self) -> &mut FrameBuffer{
            &mut self.framebuffer
        }
        pub fn should_close(&self) -> bool{
            !self.window.is_open()
        }

        pub fn display(&mut self) {
            self.window
                .update_with_buffer(&self.framebuffer.data, self.framebuffer.width, self.framebuffer.height)
                .expect("Failed to update window buffer");

            let (width, height) = self.window.get_size();
            if width != self.framebuffer.width() || height != self.framebuffer.height(){
                self.framebuffer = FrameBuffer::new(height, width)
            }
        }

        pub fn check_key(&mut self, key: minifb::Key) -> bool{
            if self.window.is_key_down(key) {
                return true
            }else{
                return false
            }
        }

}

impl FrameBuffer{
    pub fn new (height: usize, width: usize) -> Self{
        FrameBuffer { data: vec![0; width*height], width, height }
    }

    pub fn width(&self) -> usize{
        self.width
    }

    pub fn height(&self) -> usize{
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32){
        self.data[x + y*self.width] = color;
    }

    pub fn clear(&mut self, color: u32){
        for i in 0..self.data.len(){
            self.data[i] = color;
        }
    }

    
}