use std::time::Instant;
use std::{error::Error, fs, time::Duration};

// sdl components
use crate::view::display::{ColorTheme, DisplayHandler};
use crate::view::input::InputHandler;
use crate::view::sound::SoundHandler;

// emulator components
use crate::{cpu::Cpu, memory::Memory, timer::Timer, vmemory::VMemory, rng::RandomByte};

pub const DEFAULT_CPU_CLOCK: u64 = 600;

pub fn cpu_clock_from_str(str: &str) -> Result<u64, String> {
    match str.parse::<u64>().ok() {
        Some(s @ 300..=1000) => Ok(s),
        _ => Err(format!(
            "[clock] must be an Integer within [300, 1000]. You provided \"{}\"",
            str
        )),
    }
}

pub struct Config {
    pub program_filename: String,
    pub theme: ColorTheme,
    pub scale: u32,
    pub cpu_clock: u64,
    pub muted: bool,
}

pub fn emulate_chip8(config: Config) -> Result<(), Box<dyn Error>> {
    // initialize view
    let sdl_context = sdl2::init().unwrap();
    let mut input = InputHandler::new(&sdl_context);
    let mut display = DisplayHandler::new(&sdl_context, config.scale, config.theme);
    let sound = SoundHandler::new(&sdl_context, config.muted);

    // read provided ROM file
    let program = fs::read(config.program_filename)?;

    // initialize timings
    let cycle_duration_timer = Duration::from_nanos(1_000_000_000 / 60);
    let cycle_duration_cpu = Duration::from_nanos(1_000_000_000 / config.cpu_clock);

    // main Loop
    'running: loop {
        // initialize emulator
        let mem = Memory::new(&program)?;
        let timer = Timer::new();
        let vmemory = VMemory::new();
        let rng = RandomByte::new();
        let mut cpu = Cpu::new(mem, timer, vmemory, rng);

        // this delays pausing the beep after it started. Otherwise it may not come through
        let mut sound_delay = 0;

        // emulate system
        'emulation: loop {
            // start time measurment
            let now = Instant::now();
            let mut elapsed = Duration::from_secs(0);

            // handle timers
            let beep = cpu.update_timers();

            if beep {
                sound_delay = 3;
                sound.resume();
            } else if sound_delay == 0 {
                sound.pause();
            }

            if sound_delay > 0 {
                sound_delay -= 1;
            }

            // cycle the cpu at the specified timings until the next tick of the timers
            'cpu: loop {
                let input_event = input.poll();

                // Quitting takes precedence over restarting
                if input_event.quit {
                    break 'running;
                }

                // Reinitialize emulator on restart
                if input_event.restart {
                    break 'emulation;
                }

                let state = cpu.cycle(&input_event.keypad_state)?;

                match state.draw {
                    None => (),
                    Some(pixels) => display.draw(pixels)?,
                }

                // Determine cpu timings
                let cycle_time = now.elapsed() - elapsed;
                elapsed += cycle_time;

                if cycle_time < cycle_duration_cpu {
                    let sleep = cycle_duration_cpu - cycle_time;
                    elapsed += sleep;
                    std::thread::sleep(sleep);
                }

                if elapsed >= cycle_duration_timer {
                    break 'cpu;
                }
            }
        }
    }

    Ok(())
}
