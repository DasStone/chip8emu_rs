use sdl2::{render::Canvas, video::Window, Sdl, pixels::Color, rect::Rect};

use crate::vmemory::{SCREEN_WIDTH, SCREEN_HEIGHT, idx};

const RED: (u8, u8, u8, u8, u8, u8) = (255, 180, 40, 120, 8, 0);
const GREEN: (u8, u8, u8, u8, u8, u8) = (55, 255, 40, 30, 80, 0);
const BLUE: (u8, u8, u8, u8, u8, u8) = (5, 50, 90, 60, 114, 164);

const BWHITE: (u8, u8, u8, u8, u8, u8) = (255, 255, 255, 0, 0, 0);
const BRED: (u8, u8, u8, u8, u8, u8) = (255, 0, 0, 0, 0, 0);
const BGREEN: (u8, u8, u8, u8, u8, u8) = (0, 255, 0, 0, 0, 0);
const BBLUE: (u8, u8, u8, u8, u8, u8) = (0, 0, 255, 0, 0, 0);

pub struct DisplayHandler {
    canvas: Canvas<Window>,
    primary_color: Color,
    secondary_color: Color,
}

impl DisplayHandler {
    pub fn new(sdl_context: &Sdl, scale: u32) -> DisplayHandler {
        let video_subsystem = sdl_context.video().unwrap();

        let width = (SCREEN_WIDTH as u32) * scale;
        let height = (SCREEN_HEIGHT as u32) * scale;

        let window = video_subsystem.window("chip8emu_rs", width, height)
        .position_centered()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let color = BWHITE;

        DisplayHandler {
            canvas: canvas,
            primary_color: Color::RGB(color.0, color.1, color.2),
            secondary_color: Color::RGB(color.3, color.4, color.5),
        }
    }

    pub fn draw(&mut self, buffer: &Box<[u8]>, scale: usize) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if buffer[idx(x, y)] == 1 { 
                    self.canvas.set_draw_color(self.primary_color);
                } else {
                    self.canvas.set_draw_color(self.secondary_color);
                }
                self.canvas.fill_rect(Rect::new((x * scale) as i32, (y * scale) as i32, scale as u32, scale as u32));
            }
        }

        self.canvas.present();
    }
}