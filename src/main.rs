use chip8::{emulate_chip8, Config};
use clap::{App, Arg};
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
    let matches = App::new("chip8emu_rs")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Chip8 emulator")
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
                .help("Color Theme: r, g, b, br, bg, bb, bw")
                .takes_value(true),
        )
        .arg(Arg::with_name("SCALE")
                .short("s")
                .long("scale")
                .help("Scales pixel size")
                .takes_value(true)
        )
        .get_matches();

    let scale = value_t!(matches, "SCALE", u32).unwrap_or(10);
    let muted = matches.is_present("MUTED");

    println!("Scale is: {}", scale);
    println!("Mute: {}", muted);

    let mut theme = "bw";
    if let Some(ov) = matches.value_of("THEME") {
        theme = ov;
    }

    let config = Config {
        program_filename: "Space Invaders [David Winter].ch8".to_string(),
        theme: theme.to_string(),
        scale: 10,
        muted: false,
    };

    if let Err(err) = emulate_chip8(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }

    //emulate_chip8("Russian Roulette [Carmelo Cortez, 1978].ch8");
    //emulate_chip8("invaders.c8");
    //emulate_chip8("test_opcode.ch8");
    //emulate_chip8("c8_test.c8");
}
