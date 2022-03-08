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
    // Initialize view
    let sdl_context = sdl2::init().unwrap();
    let mut input = InputHandler::new(&sdl_context);
    let mut display = DisplayHandler::new(&sdl_context, config.scale, config.theme);
    let sound = SoundHandler::new(&sdl_context, config.muted);

    // Read provided ROM file
    let program = fs::read(config.program_filename)?;

    // Main Loop
    'running: loop {
        // Initialize emulator
        let mem = Memory::new(&program)?;
        let timer = Timer::new();
        let vmemory = VMemory::new();
        let rng = RandomByte::new();
        let mut cpu = Cpu::new(mem, timer, vmemory, rng);

        // emulate system
        'emulation: loop {
            let input_event = input.poll();

            // Quitting takes precedence over restarting
            if input_event.quit {
                break 'running
            }

            // Reinitialize emulator on restart
            if input_event.restart {
                break 'emulation
            }

            let state = cpu.cycle(input_event)?;

            if state.beep {
                sound.resume();
            } else {
                sound.pause();
            }

            match state.draw {
                None => (),
                Some(pixels) => display.draw(pixels)?,
            }
            
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    Ok(())
}

