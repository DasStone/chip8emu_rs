use std::time::Instant;
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

    // Initialize Timings
    let cycle_duration_timer: u128 = 1_000_000_000 / 60;
    let cycle_duration_cpu:u128 = 1_000_000_000 / 700;

    // Main Loop
    'running: loop {
        // Initialize emulator
        let mem = Memory::new(&program)?;
        let timer = Timer::new();
        let vmemory = VMemory::new();
        let rng = RandomByte::new();
        let mut cpu = Cpu::new(mem, timer, vmemory, rng);

        let mut sound_delay = 0;

        let truth = Instant::now();
        let mut counter = 0;

        // emulate system
        'emulation: loop {

            let now = Instant::now();
            let mut elapsed = 0;

            // handle timers
            //println!("timers");
            let beep = cpu.update_timers();

            if beep {
                sound_delay = 5;
                sound.resume();
            } else if sound_delay == 0 {
                sound.pause();
            }

            if sound_delay > 0 {
                sound_delay -= 1;
            }

            // if counter == 60 {
            //     println!("Time: {}", truth.elapsed().as_millis());
            //     counter = 0;
            //     //panic!();
            // }

            // counter += 1;

            'cpu: loop {
                //println!("cpu");

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

                match state.draw {
                    None => (),
                    Some(pixels) => display.draw(pixels)?,
                }

                let cycle_time = now.elapsed().as_nanos() - elapsed;
                elapsed += cycle_time;

                if cycle_time < cycle_duration_cpu {
                    let sleep = cycle_duration_cpu - cycle_time;
                    elapsed += sleep;
                    std::thread::sleep(Duration::from_nanos(sleep as u64));
                }

                //println!("elapsed {} ----- cycle_duration_timer {}", elapsed, cycle_duration_timer);

                if elapsed >= cycle_duration_timer {
                    break 'cpu;
                }
            } 

            //println!("{}", now.elapsed().as_millis());
        }
    }

    Ok(())
}
