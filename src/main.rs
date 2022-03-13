use clap::{App, Arg};
use std::process;

use chip8::{cpu_clock_from_str, emulate_chip8, Config, DEFAULT_CPU_CLOCK};
use view::display::{scale_from_str, theme_from_str, DEFAULT_SCALE, DEFAULT_THEME};

// emulator components
mod chip8;
mod cpu;
mod memory;
mod rng;
mod timer;
mod vmemory;

// sdl interface components
mod view;

#[macro_use]
extern crate clap;
extern crate sdl2;

fn terminate_with_error<T: std::fmt::Display>(e: T) -> ! {
    eprintln!("Application error: {}", e);
    process::exit(1)
}

fn main() {
    // define CLI
    let matches = App::new("chip8emu_rs")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Chip8 emulator")
        .after_help("Quit the emulator by pressing <ESC>, restart by pressing <SPACE>

Input mapping:
Emulator     Chip8
+-+-+-+-+    +-+-+-+-+
|1|2|3|4|    |1|2|3|C|
|Q|W|E|R|    |4|5|6|D|
|A|S|D|F|    |7|8|9|E|
|Z|X|C|V|    |A|0|B|F|
+-+-+-+-+    +-+-+-+-+

(The US Layout is just a reference. The physical keys are used, not the values they are assigned to)")
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
                .help("Color Theme: r, g, b, br, bg, bb, bw. Default is bw")
                .takes_value(true),
        )
        .arg(Arg::with_name("SCALE")
                .short("s")
                .long("scale")
                .help("Scales pixel size. Valid Range: [1, 100]. Default is 10")
                .takes_value(true)
        )
        .arg(Arg::with_name("CLOCK")
                .short("c")
                .long("clock")
                .help("Sets CPU clock speed (in Hz). Valid Range: [500, 1000]. Default is 600")
                .takes_value(true)
        )
        .get_matches();

    // parse Arguments
    let filename = matches.value_of("ROM").unwrap();

    let muted = matches.is_present("MUTED");

    let scale = if let Some(ov) = matches.value_of("SCALE") {
        scale_from_str(ov).unwrap_or_else(|err| terminate_with_error(err))
    } else {
        DEFAULT_SCALE
    };

    let theme = if let Some(ov) = matches.value_of("THEME") {
        theme_from_str(ov).unwrap_or_else(|err| terminate_with_error(err))
    } else {
        DEFAULT_THEME
    };

    let cpu_clock = if let Some(ov) = matches.value_of("CLOCK") {
        cpu_clock_from_str(ov).unwrap_or_else(|err| terminate_with_error(err))
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
        terminate_with_error(err)
    }
}
