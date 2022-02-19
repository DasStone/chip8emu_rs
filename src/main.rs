mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;

use cpu::Cpu;
use sound::get_sound_handler;
use sound::SoundMode;
use timer::Timer;

fn main() {
    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let sound = get_sound_handler(SoundMode::Print);
    let timer = Timer::new(sound);

    let mut cpu = Cpu::new(mem, timer);
    cpu.cycle();

    println!("-----------");
}


