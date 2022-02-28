#[derive(Clone)]
pub struct Timer {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        let mut beep = false;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            beep = true;

            self.sound_timer -= 1;
        }

        beep
    }
}
