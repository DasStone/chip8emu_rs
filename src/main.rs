mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;
mod rng;
mod display;

use std::fs;

use cpu::Cpu;
use sound::get_sound_handler;
use sound::SoundMode;
use timer::Timer;

extern crate sdl2;

use crate::display::Display;

fn main() {
    //let program = fs::read("IBM Logo.ch8").expect("oof");
    let mut mem = memory::Memory::new(&vec![4u8; 200]).expect("rip");
    let sound = get_sound_handler(SoundMode::Print);
    let timer = Timer::new(sound);
    let rng = rng::RandomByte::new();
    let display = Display::new(); 

    let mut cpu = Cpu::new(mem, timer, display, rng);

    for n in 0..200 {
        cpu.cycle();
    }

    cpu.debug_print();
    


    println!("-----------");
}


