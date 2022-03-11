use chip8::{emulate_chip8, Config, cpu_clock_from_string, DEFAULT_CPU_CLOCK};
use clap::{App, Arg};
use display::{theme_of_str, scale_of_str, DEFAULT_SCALE, DEFAULT_THEME};
use std::process;

mod chip8;
mod cpu;
mod display;
mod input;
mod memory;
mod rng;
mod sound;
mod timer;
mod vmemory;

#[macro_use]
extern crate clap;
extern crate sdl2;

fn main() {
    // define CLI
    let matches = App::new("chip8emu_rs")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Chip8 emulator")
        .after_help("Quit the emulator at any time by pressing ESC. Restart the emultaor by pressing SPACE.")
        .version_short("v")
        .arg(
            Arg::with_name("ROM")
                .help("Filename of the chip8-program")
                .required(true),
        )
        .arg(
            Arg::with_name("MUTED")
                .short("m")
                .long("mute")
                .help("Mutes emulator audio"),
        )
        .arg(
            Arg::with_name("THEME")
                .short("t")
                .long("theme")
                .help("Color Theme: r, g, b, br, bg, bb, bw. Default is bw.")
                .takes_value(true),
        )
        .arg(Arg::with_name("SCALE")
                .short("s")
                .long("scale")
                .help("Scales pixel size. Valid Range: [1, 100]. Default is 10.")
                .takes_value(true)
        )
        .arg(Arg::with_name("CLOCK")
                .short("c")
                .long("clock")
                .help("Sets CPU clock speed. Valid Range: [500, 1000]. Default is 600.")
                .takes_value(true)
        )
        .get_matches();

    // parse Arguments
    let filename = matches.value_of("ROM").unwrap();

    let muted = matches.is_present("MUTED");

    let scale =  if let Some(ov) = matches.value_of("SCALE") {
        scale_of_str(ov).unwrap_or_else(|err| {
            eprintln!("Application error: {}", err);
            process::exit(1);
        })
    } else {
        DEFAULT_SCALE
    };

    let theme = if let Some(ov) = matches.value_of("THEME") {
        theme_of_str(ov).unwrap_or_else(|err| {
            eprintln!("Application error: {}", err);
            process::exit(1);
        })
    } else {
        DEFAULT_THEME
    };

    let cpu_clock = if let Some(ov) = matches.value_of("CLOCK") {
        cpu_clock_from_string(ov).unwrap_or_else(|err| {
            eprintln!("Application error: {}", err);
            process::exit(1);
        })
    } else {
        DEFAULT_CPU_CLOCK
    };

    // create emulator configuration
    let config = Config {
        program_filename: filename.to_string(),
        theme: theme,
        scale: scale,
        cpu_clock: cpu_clock,
        muted: muted,
    };

    // start emulator
    if let Err(err) = emulate_chip8(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
