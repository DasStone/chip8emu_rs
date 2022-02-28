mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;
mod rng;
mod display;

use cpu::Cpu;
use timer::Timer;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::time::Duration;

use crate::display::Display;
use crate::sound::SoundHandler;

fn main() {
    //let program = fs::read("IBM Logo.ch8").expect("oof");
    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let timer = Timer::new();
    let rng = rng::RandomByte::new();
    let display = Display::new(); 

    let mut cpu = Cpu::new(mem, timer, display, rng);

    for n in 0..200 {
        cpu.cycle();
    }

    cpu.debug_print();
    


    println!("-----------");

    let sdl_context = sdl2::init().unwrap();

    let sound = SoundHandler::new(&sdl_context, false);

    sound.resume();

    std::thread::sleep(Duration::from_millis(1000));

    sound.resume();

    std::thread::sleep(Duration::from_millis(500));

    sound.resume();

    std::thread::sleep(Duration::from_millis(500));

    sound.pause();



    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("chip8emu_rs", 800, 600)
    //     .position_centered()
    //     .resizable()
    //     //.fullscreen()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }
}


