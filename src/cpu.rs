use crate::{memory::{PROGRAM_START, Memory}, timer::Timer};

#[derive(Clone)]
pub struct Cpu {
    i: u16,
    pc: u16,
    opcode: u16,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],

    memory: Memory,
    timer: Timer,

}

impl Cpu {
    pub fn new(memory: Memory, timer: Timer) -> Cpu {
        Cpu {
            i: 0x0,
            pc: PROGRAM_START as u16,
            opcode: 0x0,
            sp: 0,
            stack: [0x0; 16],
            v: [0x0; 16],
            memory: memory,
            timer: timer,
        }
    }

    pub fn cycle(&mut self) -> Result<(), String>{
        // fetch, decode and execute instruction
        let op_code= self.fetch();
        self.decode_and_execute(op_code)?;

        // update timers
        self.timer.update();

        Ok(())
    }

    fn fetch(&self) -> u16 {
        let pc = self.pc as usize;
        let first_byte = self.memory.mem[pc] as u16;
        let second_byte = self.memory.mem[pc + 1] as u16;
        
        first_byte << 8 | second_byte
    }

    fn decode_and_execute(&mut self, op_code: u16) -> Result<(), String>{
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
                0x9E => self.op_ex9e(x),
                0xA1 => self.op_exa1(x),
                _ => unrecognized = true,
            }
            0xF000 => match op_code & 0x00FF {
                0x07 => self.op_fx07(x),
                0x0A => self.op_fx0a(x),
                0x15 => self.op_fx15(x),
                0x18 => self.op_fx18(x),
                0x1E => self.op_fx1e(x),
                0x29 => self.op_fx29(x),
                0x33 => self.op_fx33(x),
                0x55 => self.op_fx55(x),
                0x65 => self.op_fx65(x),
                _ => unrecognized = true,
            }
            _ => unrecognized = true,
        }

        if unrecognized {
            return Err(format!("Instruction unknown {}", op_code));
        }

        Ok(())
    }

    fn op_00e0(&mut self) {}

    fn op_00ee(&mut self) {}

    fn op_0nnn(&mut self, nnn: u16) {}

    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {}

    fn op_3xkk(&mut self, x: usize, kk: u8) {}

    fn op_4xkk(&mut self, x: usize, kk: u8) {}

    fn op_5xy0(&mut self, x: usize, y: usize) {}

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {}

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

    fn op_8xy4(&mut self, x: usize, y: usize) {}

    fn op_8xy5(&mut self, x: usize, y: usize) {}

    fn op_8xy6(&mut self, x: usize, y: usize) {}

    fn op_8xy7(&mut self, x: usize, y: usize) {}

    fn op_8xye(&mut self, x: usize, y: usize) {}

    fn op_9xy0(&mut self, x: usize, y: usize) {}

    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + (self.v[0] as u16);
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {}

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {}

    fn op_ex9e(&mut self, x: usize) {}

    fn op_exa1(&mut self, x: usize) {}

    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.timer.delay_timer;
    }

    fn op_fx0a(&mut self, x: usize) {}

    fn op_fx15(&mut self, x: usize) {
        self.timer.delay_timer = self.v[x];
    }

    fn op_fx18(&mut self, x: usize) {
        self.timer.sound_timer = self.v[x];
    }

    fn op_fx1e(&mut self, x: usize) {
        self.i = self.i + (self.v[x] as u16);
    }

    fn op_fx29(&mut self, x: usize) {}

    fn op_fx33(&mut self, x: usize) {}

    fn op_fx55(&mut self, x: usize) {}

    fn op_fx65(&mut self, x: usize) {}
}

