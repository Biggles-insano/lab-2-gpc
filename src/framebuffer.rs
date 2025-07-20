use raylib::prelude::*;

pub const WIDTH: i32 = 100;
pub const HEIGHT: i32 = 100;
pub const SCALE: i32 = 6; // 6 píxeles por celda

pub struct Framebuffer {
    pub buffer: Vec<Vec<bool>>,
}

impl Framebuffer {
    pub fn new() -> Self {
        Framebuffer {
            buffer: vec![vec![false; WIDTH as usize]; HEIGHT as usize],
        }
    }

    pub fn point(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
        d.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, color);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if self.buffer[y as usize][x as usize] {
                    Color::WHITE
                } else {
                    Color::BLACK
                };
                self.point(d, x, y, color);
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < WIDTH as usize && y < HEIGHT as usize {
            self.buffer[y][x] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < WIDTH as usize && y < HEIGHT as usize {
            self.buffer[y][x]
        } else {
            false
        }
    }
}