mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;
mod rng;

use cpu::Cpu;
use rand::thread_rng;
use sound::get_sound_handler;
use sound::SoundMode;
use timer::Timer;

fn main() {
    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let sound = get_sound_handler(SoundMode::Print);
    let timer = Timer::new(sound);
    let rng = rng::RandomByte::new();


    let mut cpu = Cpu::new(mem, timer, rng);
    cpu.cycle();

    println!("-----------");
}


