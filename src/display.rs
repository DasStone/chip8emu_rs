const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Clone)]
pub struct Display {
    buffer: [u8; 64 * 32],
    draw_flag: bool,
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
}