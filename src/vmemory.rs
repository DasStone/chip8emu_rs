pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[inline]
pub fn idx(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

#[derive(Clone)]
pub struct VMemory {
    pub buffer: Box<[u8]>,
    pub draw_flag: bool,
}

impl VMemory {
    pub fn new() -> VMemory {
        let tmp = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT].into_boxed_slice();

        VMemory {
            buffer: tmp,
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

            let new_pixel = (byte & mask) >> (7 - n);
            let old_pixel = self.buffer[idx(x, y)];
            //let draw = self.buffer[idx(x, y)] ^ pixel;
            self.buffer[idx(x, y)] = old_pixel ^ new_pixel;

            vf |= new_pixel & old_pixel;
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
