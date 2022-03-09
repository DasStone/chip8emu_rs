use chip8::{emulate_chip8, Config};
use clap::{App, Arg};
use display::{default_theme, theme_of_str, default_scale, scale_of_str, ColorTheme};
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
                .short("c")
                .long("color")
                .help("Color Theme: r, g, b, br, bg, bb, bw. Default is bw.")
                .takes_value(true),
        )
        .arg(Arg::with_name("SCALE")
                .short("s")
                .long("scale")
                .help("Scales pixel size. Valid Range: [1, 100]. Default is 10.")
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
        default_scale()
    };

    let theme = if let Some(ov) = matches.value_of("THEME") {
        theme_of_str(ov).unwrap_or_else(|err| {
            eprintln!("Application error: {}", err);
            process::exit(1);
        })
    } else {
        default_theme()
    };

    // create emulator configuration
    let config = Config {
        program_filename: filename.to_string(),
        theme: theme,
        scale: scale,
        muted: muted,
    };

    // start emulator
    if let Err(err) = emulate_chip8(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
