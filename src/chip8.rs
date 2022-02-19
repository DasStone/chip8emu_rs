use std::{error::Error, fs};

use crate::memory::Memory;

pub fn emulate_chip8(program_file: &str) -> Result<(), Box<dyn Error>> {
    // Initialize System
    let program = fs::read(program_file)?;
    let memory = Memory::new(&program)?;

    Ok(())
}
