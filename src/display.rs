const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[inline]
fn idx(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

#[derive(Clone)]
pub struct Display {
    buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub draw_flag: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [0x0; SCREEN_WIDTH * SCREEN_HEIGHT],
            draw_flag: true,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0x0);
        self.draw_flag = true;
    }

    pub fn normalize_coordinates(x: u8, y: u8) -> (usize, usize) {
        (x as usize % SCREEN_WIDTH, y as usize % SCREEN_HEIGHT)
    }

    pub fn draw_byte_no_wrap(&mut self, mut x: usize, y: usize, byte: u8) -> u8{
        if y >= SCREEN_HEIGHT {
            return 0;
        }

        let mut mask: u8 = 0b10000000;
        let mut vf: u8 = 0;

        for n in 0..8 {
            if x >= SCREEN_WIDTH {
                return vf;
            }

            let pixel = (byte & mask) >> (7 - n);
            let draw = self.buffer[idx(x, y)] ^ pixel;
            self.buffer[idx(x, y)] = draw;

            vf |= draw;
            x += 1;
            mask >>= 1;
        }

        vf
    }

    pub fn debug_print_buffer(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                print!("{}", if self.buffer[idx(x, y)] == 1 { '@' } else { ' ' });
            }
            println!("");
        }
    }
}