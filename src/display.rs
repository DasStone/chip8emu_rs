pub mod display {
    pub struct Display {
        buffer: [u8; 64 * 32],
    }

    impl Display {
        pub fn new() -> Display {
            Display {
                buffer: [0x0; 64 * 32],
            }
        }

        pub fn clear(&mut self) {
            self.buffer.fill(0x0);
        }
    }
}