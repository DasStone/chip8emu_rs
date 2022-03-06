use chip8::{emulate_chip8, Config};
use std::process;

mod chip8;
mod cpu;
mod timer;
mod sound;
mod display;
mod vmemory;
mod memory;
mod rng;
mod input;

extern crate sdl2;

fn main() {
    // TODO: parse config from args
    let config = Config {
        program_filename: "Space Invaders [David Winter].ch8".to_string(),
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

