mod window;
use window::{Window, Framebuffer};

use glam::*;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn edge_function(a: &Vec2, c: &Vec2, b: &Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn inside_triangle(a: &Vec2, c: &Vec2, b: &Vec2, p: &Vec2) -> bool {
    let a0 = edge_function(b, c, p);
    let a1 = edge_function(c, a, p);
    let a2 = edge_function(a, b, p);

    let mut overlaps = false;
    overlaps |= a0 > 0.0 && a1 > 0.0 && a2 > 0.0;
    overlaps |= a0 < 0.0 && a1 < 0.0 && a2 < 0.0;

    overlaps
}

static POINTS: &[Vec2] = &[
    Vec2::new(0.3, 0.3),
    Vec2::new(0.7, 0.3),
    Vec2::new(0.5, 0.7)
];

fn main(){
    let mut window = Window::new("hm.", 512, 512);

    while window.is_open() {
        let framebuffer = window.framebuffer();

        for x in 0..framebuffer.width() {
            for y in 0..framebuffer.height() {
                let p = Vec2::new(x as f32 / framebuffer.width() as f32, y as f32 / framebuffer.height() as f32);

                if inside_triangle(&POINTS[0], &POINTS[1], &POINTS[2], &p) {
                    framebuffer.set_pixel(x, y, from_u8_rgb(255, 0, 0));
                } else {
                    framebuffer.set_pixel(x, y, from_u8_rgb(0, 0, 0));
                }
            }
        }

        window.display();
    }
}