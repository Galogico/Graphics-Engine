mod window;
use std::mem::zeroed;

use window::Window;

use glam::*;
use crate::window::FrameBuffer;
use minifb;

use image::DynamicImage;



fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn from_u8_rgba(r: u8, g: u8, b: u8, a:u8) -> u32 {
    let (r, g, b, a) = (r as u32, g as u32, b as u32, a as u32);
    (a << 24) | (r << 16) | (g << 8) | b
}

fn edging_fn(a:&Vec2, c:&Vec2, b:&Vec2) -> f32{
    ((c.x - a.x) * (b.y -a.y)) - ((c.y - a.y) * (b.x - a.x))
}

fn inside_triangle(a:&Vec2, b:&Vec2,c:&Vec2, point:&Vec2) -> bool{
    let a0 = edging_fn(b, c, point);
    let a1 = edging_fn(c, a, point);
    let a2 = edging_fn(a, b, point);
    let mut overlap = true;
    overlap &= a0 > 0.0;
    overlap &= a1 > 0.0;
    overlap &= a2 > 0.0;

    return overlap;
}

fn draw_triangle(framebuffer: &mut FrameBuffer, a:&Vec2, b:&Vec2, c:&Vec2, color: u32){
    let width = framebuffer.width();
    let height = framebuffer.height();

    let min = a.min(b.min(*c)).max(Vec2::ZERO) * Vec2::new(width as f32, height as f32);
    let max = a.max(b.max(*c)).min(Vec2::ONE) * Vec2::new(width as f32, height as f32);

        for x in (min.x as usize)..(max.x as usize) {
            for y in (min.y as usize)..(max.y as usize) {
                let point = Vec2::new(x as f32 / width as f32, y as f32 / height as f32);

                let inside = inside_triangle(a, b, c, &point);
                if inside{
                    framebuffer.set_pixel(x, y, color);
                }
            }
        }
}

fn draw_image(img: &DynamicImage, framebuffer: &mut FrameBuffer){
    let rgba_img = img.to_rgba8();

    for (x,y, pixel) in rgba_img.enumerate_pixels(){
        let r = pixel[0] as u8;
        let g = pixel[1] as u8;
        let b = pixel[2] as u8;
        let a = pixel[3] as u8;

        framebuffer.set_pixel(x as usize, y as usize, from_u8_rgba(r, g, b, a));
    }
}

// static POINTS: &[Vec2] = &[
//     Vec2::new(0.3, 0.3),
//     Vec2::new(0.7, 0.3),
//     Vec2::new(0.5, 0.7)
// ];

fn main() {
    let mut image = image::open("bg.png").expect("could not get an image");


    let mut points: Vec<Vec2> = vec![
    Vec2::new(0.3, 0.3),
    Vec2::new(0.7, 0.3),
    Vec2::new(0.5, 0.7)
    ];
    
    
    let mut  window = Window::new("gfx", 512, 512);
    
    while !window.should_close(){
        let framebuffer = window.frame_buffer();
        framebuffer.clear(from_u8_rgb(0, 0, 20));
        
        
        draw_image(&image, framebuffer);
        draw_triangle(framebuffer, &points[0], &points[1], &points[2], from_u8_rgb(255, 0, 0));

        if window.check_key(minifb::Key::A){
            points[0] = points[0] + Vec2::new(-0.005, 0.0);
            points[1] = points[1] + Vec2::new(-0.005, 0.0);
            points[2] = points[2] + Vec2::new(-0.005, 0.0);
        }
        if window.check_key(minifb::Key::D){
            points[0] = points[0] + Vec2::new(0.005, 0.0);
            points[1] = points[1] + Vec2::new(0.005, 0.0);
            points[2] = points[2] + Vec2::new(0.005, 0.0);
        }
        if window.check_key(minifb::Key::W){
            points[0] = points[0] + Vec2::new(0.0, -0.005);
            points[1] = points[1] + Vec2::new(0.0, -0.005);
            points[2] = points[2] + Vec2::new(0.0, -0.005);
        }
        if window.check_key(minifb::Key::S){
            points[0] = points[0] + Vec2::new(0.0, 0.005);
            points[1] = points[1] + Vec2::new(0.0, 0.005);
            points[2] = points[2] + Vec2::new(0.0, 0.005);
        }

        window.display();

    }
}
