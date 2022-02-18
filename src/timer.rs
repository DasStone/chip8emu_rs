#[derive(Clone)]
pub struct Timer {
    pub delay_timer: u8,
    pub sound_timer: u8,

    sound_handler: fn(),
}

impl Timer {
    pub fn new(sound_handler: fn()) -> Timer {
        Timer {
            delay_timer: 0,
            sound_timer: 0,
            sound_handler: sound_handler,
        }
    }

    pub fn update(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                (self.sound_handler)();
            }

            self.sound_timer -= 1;
        }
    }
}
