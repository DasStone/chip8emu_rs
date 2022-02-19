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
    let mut cpu = Cpu::new();
    cpu.cycle();
    cpu.cycle();

    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    

    let sound = get_sound_handler(SoundMode::On);
    let mut timer = Timer::new(sound);

    timer.sound_timer = 5;
    timer.update();
    timer.update();
    timer.update();
    timer.update();
    timer.update();


    println!("-----------");
    for n in 0..90 {
        println!("{}", mem.memory[n]);
    }
    println!("-----------");

    timer.update();
}


