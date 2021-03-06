use crate::vmemory::{idx, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub type ColorTheme = (u8, u8, u8, u8, u8, u8);

const RED: ColorTheme = (255, 180, 40, 120, 8, 0);
const GREEN: ColorTheme = (55, 255, 40, 30, 80, 0);
const BLUE: ColorTheme = (5, 50, 90, 60, 114, 164);
const BRED: ColorTheme = (255, 0, 0, 0, 0, 0);
const BGREEN: ColorTheme = (0, 255, 0, 0, 0, 0);
const BBLUE: ColorTheme = (0, 0, 255, 0, 0, 0);
const BWHITE: ColorTheme = (255, 255, 255, 0, 0, 0);

pub const DEFAULT_THEME: ColorTheme = BWHITE;

pub const DEFAULT_SCALE: u32 = 10;

pub fn theme_from_str(str: &str) -> Result<ColorTheme, String> {
    let t: ColorTheme = match str {
        "r" => RED,
        "g" => GREEN,
        "b" => BLUE,
        "br" => BRED,
        "bg" => BGREEN,
        "bb" => BBLUE,
        "bw" => BWHITE,
        _ => {
            return Err(format!(
                "[theme] \"{}\" is not known. Try --help or -h for a list of themes",
                str
            ))
        }
    };
    Ok(t)
}

pub fn scale_from_str(str: &str) -> Result<u32, String> {
    match str.parse::<u32>().ok() {
        Some(s @ 1..=100) => Ok(s),
        _ => Err(format!(
            "[scale] must be an Integer within [1, 100]. You provided \"{}\"",
            str
        )),
    }
}

pub struct DisplayHandler {
    canvas: Canvas<Window>,
    primary_color: Color,
    secondary_color: Color,
    scale: u32,
}

impl DisplayHandler {
    pub fn new(sdl_context: &Sdl, scale: u32, theme: ColorTheme) -> DisplayHandler {
        let video_subsystem = sdl_context.video().unwrap();

        let width = (SCREEN_WIDTH as u32) * scale;
        let height = (SCREEN_HEIGHT as u32) * scale;

        let window = video_subsystem
            .window("chip8emu_rs", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(theme.3, 4, 5));
        canvas.clear();
        canvas.present();

        DisplayHandler {
            canvas: canvas,
            primary_color: Color::RGB(theme.0, theme.1, theme.2),
            secondary_color: Color::RGB(theme.3, theme.4, theme.5),
            scale: scale,
        }
    }

    pub fn draw(&mut self, buffer: &Box<[u8]>) -> Result<(), String> {
        self.canvas.set_draw_color(self.secondary_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.primary_color);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if buffer[idx(x, y)] != 1 {
                    continue;
                }

                self.canvas.fill_rect(Rect::new(
                    (x * self.scale as usize) as i32,
                    (y * self.scale as usize) as i32,
                    self.scale,
                    self.scale,
                ))?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
