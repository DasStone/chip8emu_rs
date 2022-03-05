use chip8::emulate_chip8;

mod chip8;
mod cpu;
mod timer;
mod sound;
mod display;
mod vmemory;
mod memory;
mod rng;
mod keypad;

extern crate sdl2;

fn main() {
    emulate_chip8("test_opcode.ch8");
}

