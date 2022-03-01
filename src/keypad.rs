use sdl2::{EventPump, Sdl, keyboard::Keycode, event::Event};

#[derive(Clone)]
pub struct InputEvent {
    quit: bool,
    pub keypad_state: Box<[u8]>,
}

pub struct Keypad {
    events: EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &Sdl) -> Keypad {
        let events = sdl_context.event_pump().unwrap();

        Keypad {
            events: events,
        }
    }

    pub fn poll(&mut self) -> InputEvent {
        let mut quit = false;

        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                quit = true;
            }
        }

        let mut keypad_state = vec![0u8; 16].into_boxed_slice();

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            match key {
                Keycode::Num1 => keypad_state[0x0] = 1,
                Keycode::Num2 => keypad_state[0x1] = 1,
                Keycode::Num3 => keypad_state[0x2] = 1,
                Keycode::Num4 => keypad_state[0x3] = 1,
                Keycode::Q => keypad_state[0x4] = 1,
                Keycode::W => keypad_state[0x5] = 1,
                Keycode::E => keypad_state[0x6] = 1,
                Keycode::R => keypad_state[0x7] = 1,
                Keycode::A => keypad_state[0x8] = 1,
                Keycode::S => keypad_state[0x9] = 1,
                Keycode::D => keypad_state[0xA] = 1,
                Keycode::F => keypad_state[0xB] = 1,
                Keycode::Z => keypad_state[0xC] = 1,
                Keycode::X => keypad_state[0xD] = 1,
                Keycode::C => keypad_state[0xE] = 1,
                Keycode::V => keypad_state[0xF] = 1,
                _ => panic!(),
            }
        }

        InputEvent {
            quit: quit,
            keypad_state: keypad_state
        }
    }
}
