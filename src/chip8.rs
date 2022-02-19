use std::{error::Error, fs};

use crate::memory::Memory;

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

pub fn emulate_chip8(program_file: &str) -> Result<(), Box<dyn Error>> {
    // Initialize System
    let program = fs::read(program_file)?;
    let memory = Memory::new(&program)?;

    Ok(())
}
