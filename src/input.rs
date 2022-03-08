use sdl2::{EventPump, Sdl, keyboard::Scancode, event::Event};

#[derive(Clone)]
pub struct InputEvent {
    pub quit: bool,
    pub restart: bool,
    pub keypad_state: Box<[u8]>,
}

pub struct InputHandler {
    event_pump: EventPump,
}

impl InputHandler {
    pub fn new(sdl_context: &Sdl) -> InputHandler {
        let event_pump = sdl_context.event_pump().unwrap();

        InputHandler {
            event_pump: event_pump,
        }
    }

    pub fn poll(&mut self) -> InputEvent {
        let mut quit = false;
        let mut restart = false;

        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                quit = true;
            }
        }

        let mut keypad_state = vec![0u8; 16].into_boxed_slice();

        for sc in self.event_pump.keyboard_state().pressed_scancodes() {
            match sc {
                Scancode::Num1 => keypad_state[0x1] = 1,
                Scancode::Num2 => keypad_state[0x2] = 1,
                Scancode::Num3 => keypad_state[0x3] = 1,
                Scancode::Num4 => keypad_state[0xC] = 1,
                Scancode::Q => keypad_state[0x4] = 1,
                Scancode::W => keypad_state[0x5] = 1,
                Scancode::E => keypad_state[0x6] = 1,
                Scancode::R => keypad_state[0xD] = 1,
                Scancode::A => keypad_state[0x7] = 1,
                Scancode::S => keypad_state[0x8] = 1,
                Scancode::D => keypad_state[0x9] = 1,
                Scancode::F => keypad_state[0xE] = 1,
                Scancode::Z => keypad_state[0xA] = 1,
                Scancode::X => keypad_state[0x0] = 1,
                Scancode::C => keypad_state[0xB] = 1,
                Scancode::V => keypad_state[0xF] = 1,
                Scancode::Escape => quit = true,
                Scancode::Space => restart = true,
                _ => (),
            }
        }

        InputEvent {
            quit,
            restart,
            keypad_state,
        }
    }
}
