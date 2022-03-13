pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[inline]
pub fn idx(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

#[inline]
pub fn normalize_coordinates(x: u8, y: u8) -> (usize, usize) {
    (x as usize % SCREEN_WIDTH, y as usize % SCREEN_HEIGHT)
}

#[derive(Clone, Debug)]
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
        self.draw_flag = true;
        self.buffer.fill(0x0);
    }

    pub fn draw_sprite_no_wrap(&mut self, x_pos: u8, y_pos: u8, sprite: &[u8]) -> u8 {
        self.draw_flag = true;

        // normalize coordinates
        let (x, mut curr_y) = normalize_coordinates(x_pos, y_pos);

        // flag value
        let mut vf = 0;

        // iterate sprite rows
        for byte in sprite {
            // return flag when the end of the screen y direction was reached
            if curr_y >= SCREEN_HEIGHT {
                return vf;
            }

            // x cannot be used directly, because all rows should start at the same x position
            let mut curr_x = x;
            let mut mask: u8 = 0b10000000;

            // iterate sprite columns
            for n in 0..8 {
                // continue with the next row if the end of this one is reached
                if curr_x >= SCREEN_WIDTH {
                    break;
                }
    
                // determine new pixel value
                let new_pixel = (*byte & mask) >> (7 - n);
                let old_pixel = self.buffer[idx(curr_x, curr_y)];
                self.buffer[idx(curr_x, curr_y)] = old_pixel ^ new_pixel;
    
                // set flag and update mask and current drawing position (x)
                vf |= new_pixel & old_pixel;
                mask >>= 1;
                curr_x += 1;
            }

            // update current drawing position (y)
            curr_y += 1;
        }

        vf
    }
}
