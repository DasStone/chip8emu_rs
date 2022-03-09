use crate::{
    input::InputEvent,
    memory::{Memory, FONTSET_ADDRESS, PROGRAM_START},
    rng::RandomByte,
    timer::Timer,
    vmemory::VMemory,
};

#[derive(Clone)]
pub struct EmulatorState<'a> {
    pub beep: bool,
    pub draw: Option<&'a Box<[u8]>>,
}

#[derive(Clone, Debug)]
pub struct Cpu {
    i: u16,
    pc: u16,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],

    memory: Memory,
    timer: Timer,
    vmemory: VMemory,

    rng: RandomByte,
}

impl Cpu {
    pub fn new(memory: Memory, timer: Timer, vmemory: VMemory, rng: RandomByte) -> Cpu {
        Cpu {
            i: 0x0,
            pc: PROGRAM_START as u16,
            sp: 0,
            stack: [0x0; 16],
            v: [0x0; 16],

            memory: memory,
            timer: timer,
            vmemory: vmemory,

            rng: rng,
        }
    }

    pub fn cycle<'a>(&'a mut self, input: InputEvent) -> Result<EmulatorState<'a>, String> {
        // fetch, decode and execute instruction
        let op_code = self.fetch();
        self.decode_and_execute(op_code, input)?;

        // update timers
        let beep = self.timer.update();

        // update draw state
        let draw = if self.vmemory.draw_flag {
            self.vmemory.draw_flag = false;
            Some(&self.vmemory.buffer)
        } else {
            None
        };

        // return emulator state
        Ok(EmulatorState {
            beep: beep,
            draw: draw,
        })
    }

    pub fn _debug_print(&mut self, print_vmemory: bool, print_memory: bool) {
        println!("--------- CPU Info ---------");
        println!(
            "+ Special Registers:\nI: {}\npc: {}\nDelay Timer: {}\nSound Timer: {}",
            self.i, self.pc, self.timer.delay_timer, self.timer.sound_timer
        );
        println!("+ Stack:\nsp: {}\nstack: {:?}", self.pc, self.stack);
        println!("+ GP Registers:\n{:?}", self.v);
        if print_vmemory {
            println!("------- Video Memory -------");
            self.vmemory._debug_print_buffer();
        }
        if print_memory {
            println!("---------- Memory ----------");
            println!("{:?}", self.memory);
        }
        println!("----------------------------");
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        let first_byte = self.memory.mem[pc] as u16;
        let second_byte = self.memory.mem[pc + 1] as u16;

        // increment program counter, doing this here avoids duplication later on
        self.pc += 2;

        first_byte << 8 | second_byte
    }

    fn decode_and_execute(&mut self, op_code: u16, input: InputEvent) -> Result<(), String> {
        let x = ((op_code & 0x0F00) >> 8) as usize;
        let y = ((op_code & 0x00F0) >> 4) as usize;
        let nnn = (op_code & 0x0FFF) as u16;
        let kk = (op_code & 0x00FF) as u8;
        let n = (op_code & 0x000F) as u8;

        let mut unrecognized = false;

        match op_code & 0xF000 {
            0x0000 => match op_code {
                0x00E0 => self.op_00e0(),
                0x00EE => self.op_00ee(),
                _ => unrecognized = true,
            },
            0x1000 => self.op_1nnn(nnn),
            0x2000 => self.op_2nnn(nnn),
            0x3000 => self.op_3xkk(x, kk),
            0x4000 => self.op_4xkk(x, kk),
            0x5000 => self.op_5xy0(x, y),
            0x6000 => self.op_6xkk(x, kk),
            0x7000 => self.op_7xkk(x, kk),
            0x8000 => match op_code & 0x000F {
                0x0 => self.op_8xy0(x, y),
                0x1 => self.op_8xy1(x, y),
                0x2 => self.op_8xy2(x, y),
                0x3 => self.op_8xy3(x, y),
                0x4 => self.op_8xy4(x, y),
                0x5 => self.op_8xy5(x, y),
                0x6 => self.op_8xy6(x, y),
                0x7 => self.op_8xy7(x, y),
                0xE => self.op_8xye(x, y),
                _ => unrecognized = true,
            },
            0x9000 => self.op_9xy0(x, y),
            0xA000 => self.op_annn(nnn),
            0xB000 => self.op_bnnn(nnn),
            0xC000 => self.op_cxkk(x, kk),
            0xD000 => self.op_dxyn(x, y, n),
            0xE000 => match op_code & 0x00FF {
                0x9E => self.op_ex9e(x, input),
                0xA1 => self.op_exa1(x, input),
                _ => unrecognized = true,
            },
            0xF000 => match op_code & 0x00FF {
                0x07 => self.op_fx07(x),
                0x0A => self.op_fx0a(x, input),
                0x15 => self.op_fx15(x),
                0x18 => self.op_fx18(x),
                0x1E => self.op_fx1e(x),
                0x29 => self.op_fx29(x),
                0x33 => self.op_fx33(x),
                0x55 => self.op_fx55(x),
                0x65 => self.op_fx65(x),
                _ => unrecognized = true,
            },
            _ => unrecognized = true,
        }

        if unrecognized {
            return Err(format!("Instruction {} unknown", op_code));
        }

        Ok(())
    }

    fn op_00e0(&mut self) {
        self.vmemory.clear();
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn _op_0nnn(&mut self, _nnn: u16) {}

    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let (res, _) = self.v[x].overflowing_add(kk);
        self.v[x] = res;
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] | self.v[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] & self.v[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (res, carry) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = res;
        self.v[0xF] = u8::from(carry);
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (res, carry) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = res;
        self.v[0xF] = u8::from(!carry);
    }

    fn op_8xy6(&mut self, x: usize, _y: usize) {
        self.v[0xF] = self.v[x] & 0b00000001;
        self.v[x] >>= 1;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (res, carry) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = res;
        self.v[0xF] = u8::from(!carry);
    }

    fn op_8xye(&mut self, x: usize, _y: usize) {
        self.v[0xF] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
    }

    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + (self.v[0] as u16);
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.rng.sample() & kk;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {
        let start = self.i as usize;
        let end = start + (n as usize);
        self.v[0xF] = self.vmemory.draw_sprite_no_wrap(self.v[x], self.v[y], &self.memory.mem[start .. end]);
    }

    fn op_ex9e(&mut self, x: usize, input: InputEvent) {
        if input.keypad_state[self.v[x] as usize] != 0 {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self, x: usize, input: InputEvent) {
        if input.keypad_state[self.v[x] as usize] == 0 {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.timer.delay_timer;
    }

    fn op_fx0a(&mut self, x: usize, input: InputEvent) {
        let mut res = None;

        for n in 0x0..=0xF {
            if input.keypad_state[n] != 0 {
                res = Some(n as u8);
                break;
            }
        }

        match res {
            None => {
                self.pc -= 2;
                self.timer.delay_timer += 1;

                if self.timer.sound_timer > 0 {
                    self.timer.sound_timer += 1;
                }
            }
            Some(val) => self.v[x] = val,
        }
    }

    fn op_fx15(&mut self, x: usize) {
        self.timer.delay_timer = self.v[x];
    }

    fn op_fx18(&mut self, x: usize) {
        self.timer.sound_timer = self.v[x];
    }

    fn op_fx1e(&mut self, x: usize) {
        self.i = self.i + (self.v[x] as u16);
    }

    fn op_fx29(&mut self, x: usize) {
        let nibble = (self.v[x] & 0x0F) as u16;
        self.i = (FONTSET_ADDRESS as u16) + 5 * nibble;
    }

    fn op_fx33(&mut self, x: usize) {
        let tmp = self.v[x];
        self.memory.mem[self.i as usize + 0] = tmp / 100;
        self.memory.mem[self.i as usize + 1] = (tmp / 10) % 10;
        self.memory.mem[self.i as usize + 2] = tmp % 10;
    }

    fn op_fx55(&mut self, x: usize) {
        for n in 0..=x {
            self.memory.mem[(self.i as usize) + n] = self.v[n];
        }
    }

    fn op_fx65(&mut self, x: usize) {
        for n in 0..=x {
            self.v[n] = self.memory.mem[(self.i as usize) + n];
        }
    }
}
