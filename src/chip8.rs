use std::{error::Error, fs, time::Duration};

use crate::display::{DisplayHandler, ColorTheme};
use crate::rng::RandomByte;
use crate::sound::SoundHandler;
use crate::{memory::{Memory}, vmemory::VMemory, timer::Timer, input::InputHandler, cpu::Cpu};


pub struct Config {
    pub program_filename: String,
    pub theme: ColorTheme,
    pub scale: u32,
    pub muted: bool,
}

pub fn emulate_chip8(config: Config) -> Result<(), Box<dyn Error>> {
    // Initialize Emulator
    let program = fs::read(config.program_filename)?;
    let mem = Memory::new(&program)?;
    let timer = Timer::new();
    let vmemory = VMemory::new();
    let rng = RandomByte::new();
    let mut cpu = Cpu::new(mem, timer, vmemory, rng);
    
    // Initialize View
    let sdl_context = sdl2::init().unwrap();
    let mut input = InputHandler::new(&sdl_context);
    let mut display = DisplayHandler::new(&sdl_context, config.scale, config.theme);
    let sound = SoundHandler::new(&sdl_context, config.muted);

    // Main Loop
    'running: loop {
        let input_event = input.poll();

        if input_event.quit {
            break 'running
        }

        let state = cpu.cycle(input_event)?;

        if state.beep {
            sound.resume();
        } else {
            sound.pause();
        }

        match state.draw {
            None => (),
            Some(pixels) => display.draw(pixels, config.scale as usize),
        }
        
        std::thread::sleep(Duration::from_millis(4));
    }

    Ok(())
}

