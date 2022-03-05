use std::{error::Error, fs, time::Duration};

use sdl2::{pixels::Color, render::Canvas, rect::Rect};
use sdl2::video::Window;

use crate::display::Display;
use crate::{memory::{Memory}, vmemory::{VMemory, SCREEN_WIDTH, SCREEN_HEIGHT, idx}, timer::Timer, rng, keypad::Keypad, cpu::Cpu};

pub fn emulate_chip8(program_file: &str) -> Result<(), Box<dyn Error>> {
    // Initialize Emulator
    let program = fs::read(program_file).expect("oof");
    let mem = Memory::new(&program).expect("rip");
    //let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let timer = Timer::new();
    let rng = rng::RandomByte::new();
    let vmemory = VMemory::new();
    let mut cpu = Cpu::new(mem, timer, vmemory, rng);
    
    let scale = 10;
    
    // Initialize View
    let sdl_context = sdl2::init().unwrap();
    let mut keypad = Keypad::new(&sdl_context);
    let mut display = Display::new(&sdl_context, scale);

    // Main Loop
    'running: loop {

        let event = keypad.poll();

        if event.quit {
            break 'running
        }

        let state = cpu.cycle(event).expect("oof");

        match state.draw {
            None => (),
            Some(pixels) => display.draw(pixels, scale as usize),
        }
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

