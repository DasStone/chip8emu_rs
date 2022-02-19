use core::panic;

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
        let vx = ((op_code & 0x0F00) >> 8) as usize;
        let vy = ((op_code & 0x00F0) >> 4) as usize;
        let nnn = (op_code & 0x0FFF) as u16;
        let kk = (op_code & 0x00FF) as u8;
        let n = (op_code & 0x000F) as u8;
    
        let mut unrecognized = false;

        match op_code & 0xF000 {
            0x0000 => match op_code {
                0x00E0 => op_00e0(self),
                0x00EE => op_00ee(),
                _ => unrecognized = true,
            },
            0x1000 => op_1nnn(nnn),
            0x2000 => op_2nnn(nnn),
            0x3000 => op_3xkk(vx, kk),
            0x4000 => op_4xkk(vx, kk),
            0x5000 => op_5xy0(vx, vy),
            0x6000 => op_6xkk(vx, kk),
            0x7000 => op_7xkk(vx, kk),
            0x8000 => match op_code & 0x000F {
                0x0 => op_8xy0(vx, vy),
                0x1 => op_8xy1(vx, vy),
                0x2 => op_8xy2(vx, vy),
                0x3 => op_8xy3(vx, vy),
                0x4 => op_8xy4(vx, vy),
                0x5 => op_8xy5(vx, vy),
                0x6 => op_8xy6(vx, vy),
                0x7 => op_8xy7(vx, vy),
                0xE => op_8xye(vx, vy),
                _ => unrecognized = true,
            },
            0x9000 => op_9xy0(vx, vy),
            0xA000 => op_annn(nnn),
            0xB000 => op_bnnn(nnn),
            0xC000 => op_cxkk(vx, kk),
            0xD000 => op_dxyn(vx, vy, n),
            0xE000 => match op_code & 0x00FF {
                0x9E => op_ex9e(vx),
                0xA1 => op_exa1(vx),
                _ => unrecognized = true,
            }
            0xF000 => match op_code & 0x00FF {
                0x07 => op_fx07(vx),
                0x0A => op_fx0a(vx),
                0x15 => op_fx15(vx),
                0x18 => op_fx18(vx),
                0x1E => op_fx1e(vx),
                0x29 => op_fx29(vx),
                0x33 => op_fx33(vx),
                0x55 => op_fx55(vx),
                0x65 => op_fx65(vx),
                _ => unrecognized = true,
            }
            _ => unrecognized = true,
        }

        if unrecognized {
            return Err(format!("Instruction unknown {}", op_code));
        }

        Ok(())
    }
}

fn fetch() {}

fn i1nnn() {}

fn decode2() -> impl FnMut () -> () {
     || op_1nnn(0xFFF)
}


// 00E0 - CLS
// Clear the display
fn op_00e0(cpu: &mut Cpu) {
    cpu.memory.mem[0] = 69;
}

// Return from a subroutine
fn op_00ee() {}

// (panic!) Jump to machine code routine at NNN
fn _op_0nnn(nnn: u16) {}

// Jump to location NNN
fn op_1nnn(nnn: u16) {
    println!("oof");
}

// Call subroutine at NNN
fn op_2nnn(nnn: u16) {}

// Skip next instuction if VX = KK
fn op_3xkk(vx: usize, kk: u8) {}

// Skip next instruction if VX != KK
fn op_4xkk(vx: usize, kk: u8) {}

// Skip next instuction if VX = VY
fn op_5xy0(vx: usize, vy: usize) {}

// (LD) Set VX = KK
fn op_6xkk(vx: usize, kk: u8) {}

// (ADD) Set VX = KK
fn op_7xkk(vx: usize, kk: u8) {}

// (LD) Set VX = VY
fn op_8xy0(vx: usize, vy: usize) {}

// (OR) Set VX = VX OR VY
fn op_8xy1(vx: usize, vy: usize) {}

// (AND) Set VX = VX AND VY
fn op_8xy2(vx: usize, vy: usize) {}

// (XOR) Set VX = VX XOR VY
fn op_8xy3(vx: usize, vy: usize) {}

// (ADD) Set VX = VX + VY, set VF = carry
// If VX + VY is greater than 8 bits, then set VF to 1, otherwise 0. Keep lowest 8 bit as result.
fn op_8xy4(vx: usize, vy: usize) {}

// (SUB) Set VX = VX - VY, set VF = NOT borrow
// If VX > VY, then set VF to 1, otherwise 0. Subtract afterwards.
fn op_8xy5(vx: usize, vy: usize) {}

// (SHR) Set VX = VX SHR 1
// If the least-significant bit of VX is 1, then  set VF to 1, otherwise 0. Shift afterwards.
fn op_8xy6(vx: usize, vy: usize) {}

// (SUBN) Set VX = VY - VX, set VF = NOT borrow
// If VY > VX, then set VF to 1, otherwise 0. Subtract afterwards.
fn op_8xy7(vx: usize, vy: usize) {}

fn op_8xye(vx: usize, vy: usize) {}
fn op_9xy0(vx: usize, vy: usize) {}
fn op_annn(nnn: u16) {}
fn op_bnnn(nnn: u16) {}
fn op_cxkk(vx: usize, kk: u8) {}
fn op_dxyn(vx: usize, vy: usize, n: u8) {}
fn op_ex9e(vx: usize) {}
fn op_exa1(vx: usize) {}
fn op_fx07(vx: usize) {}
fn op_fx0a(vx: usize) {}
fn op_fx15(vx: usize) {}
fn op_fx18(vx: usize) {}
fn op_fx1e(vx: usize) {}
fn op_fx29(vx: usize) {}
fn op_fx33(vx: usize) {}
fn op_fx55(vx: usize) {}
fn op_fx65(vx: usize) {}

