use sdl2::{render::Canvas, video::Window, Sdl, pixels::Color, rect::Rect};

use crate::vmemory::{SCREEN_WIDTH, SCREEN_HEIGHT, idx};

pub struct Display {
    canvas: Canvas<Window>,
    primaryColor: Color,
    secondaryColor: Color,
}

impl Display {
    pub fn new(sdl_context: &Sdl, scale: u32) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let width = (SCREEN_WIDTH as u32) * scale;
        let height = (SCREEN_HEIGHT as u32) * scale;

        let window = video_subsystem.window("chip8emu_rs", width, height)
        .position_centered()
        //.resizable()
        //.fullscreen()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            canvas: canvas,
            primaryColor: Color::RGB(5, 50, 90),
            secondaryColor: Color::RGB(60, 114, 164),
        }
    }

    pub fn draw(&mut self, buffer: &Box<[u8]>, scale: usize) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if buffer[idx(x, y)] == 1 { 
                    self.canvas.set_draw_color(self.primaryColor);
                } else {
                    self.canvas.set_draw_color(self.secondaryColor);
                }
                self.canvas.fill_rect(Rect::new((x * scale) as i32, (y * scale) as i32, 10, 10));
            }
        }

        self.canvas.present();
    }
}