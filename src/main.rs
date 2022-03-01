mod chip8;
mod cpu;
mod timer;
mod sound;
mod memory;
mod rng;
mod display;
mod keypad;

use cpu::Cpu;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use sdl2::video::Window;
use timer::Timer;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::time::Duration;

use crate::display::{Display, SCREEN_WIDTH, SCREEN_HEIGHT, idx};
use crate::keypad::Keypad;
use crate::sound::SoundHandler;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    //let program = fs::read("IBM Logo.ch8").expect("oof");
    //let mem = memory::Memory::new(&program).expect("rip");
    let mem = memory::Memory::new(&vec![4u8; 100]).expect("rip");
    let timer = Timer::new();
    let rng = rng::RandomByte::new();
    let display = Display::new();

    let mut keypad = Keypad::new(&sdl_context);

    

    let mut cpu = Cpu::new(mem, timer, display, rng);

    // for n in 0..200 {
    //     let state = cpu.cycle(keypad.poll()).expect("oof");
    //     match state.draw {
    //         None => println!("no draw"),
    //         Some(pixels) => println!("{}", pixels[0]),
    //     }
    // }

    // cpu.debug_print();

    println!("-----------");

    let scale = 10;

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip8emu_rs", (SCREEN_WIDTH * scale) as u32, (SCREEN_HEIGHT * scale) as u32)
        .position_centered()
        //.resizable()
        //.fullscreen()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    //let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} |
        //         Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        //             break 'running
        //         },
        //         _ => {}
        //     }
        // }
        // The rest of the game loop goes here...

        let event = keypad.poll();

        if event.quit {
            break 'running
        }

        let state = cpu.cycle(event).expect("oof");

        match state.draw {
            None => println!("no draw"),
            Some(pixels) => draw(&mut canvas, pixels, scale),
        }
    


        println!("-----------");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw(canvas: &mut Canvas<Window>, buffer: &Box<[u8]>, scale: usize) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            if buffer[idx(x, y)] == 1 { 
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.fill_rect(Rect::new((x * scale) as i32, (y * scale) as i32, 10, 10));
        }
    }
}

