mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;
mod rng;
mod display;

use cpu::Cpu;
use sound::get_sound_handler;
use sound::SoundMode;
use timer::Timer;

extern crate sdl2;

use crate::display::Display;

fn main() {
    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let sound = get_sound_handler(SoundMode::Print);
    let timer = Timer::new(sound);
    let rng = rng::RandomByte::new();
    let display = Display::new();

    

    let mut cpu = Cpu::new(mem, timer, display, rng);
    cpu.cycle();

    println!("-----------");
}


