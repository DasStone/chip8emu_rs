use core::panic;

use crate::memory::PROGRAM_START;

#[derive(Clone)]
pub struct Cpu {
    i: u16,
    pc: u16,
    opcode: u16,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],
    memory: [u8; 4096],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0x0,
            pc: PROGRAM_START as u16,
            opcode: 0x0,
            sp: 0,
            stack: [0x0; 16],
            v: [0x0; 16],
            memory: [0x0; 4096],
        }
    }

    pub fn cycle(&mut self) {
        println!("{}", self.memory[4095]);
        self.memory[4095] = 5;

        // fetch instruction
        // decode instruction
        decode(0x00E0);
        self.fetch();
        // execute instuction
    }

    fn fetch(&mut self) {

    }
}

//fn fetch() {}

fn i1nnn() {}

// fn decode2(&memory) -> impl FnMut () -> () {
//     || i1nnn(memory)
// }

fn decode(op_code: u16) -> Instruction {
    let vx = ((op_code & 0x0F00) >> 8) as usize;
    let vy = ((op_code & 0x00F0) >> 4) as usize;
    let tribble = (op_code & 0x0FFF) as u16;
    let byte = (op_code & 0x00FF) as u8;
    let nibble = (op_code & 0x000F) as u8;

    match op_code & 0xF000 {
        0x0000 => match op_code {
            0x00E0 => Instruction::I00E0(),
            0x00EE => Instruction::I00EE(),
            _ => panic!(),
        },
        0x1000 => Instruction::I1NNN(tribble),
        0x2000 => Instruction::I2NNN(tribble),
        0x3000 => Instruction::I3XKK(vx, byte),
        0x4000 => Instruction::I4XKK(vx, byte),
        0x5000 => Instruction::I5XY0(vx, vy),
        0x6000 => Instruction::I6XKK(vx, byte),
        0x7000 => Instruction::I7XKK(vx, byte),
        0x8000 => match op_code & 0x000F {
            0x0 => Instruction::I8XY0(vx, vy),
            0x1 => Instruction::I8XY1(vx, vy),
            0x2 => Instruction::I8XY2(vx, vy),
            0x3 => Instruction::I8XY3(vx, vy),
            0x4 => Instruction::I8XY4(vx, vy),
            0x5 => Instruction::I8XY5(vx, vy),
            0x6 => Instruction::I8XY6(vx, vy),
            0x7 => Instruction::I8XY7(vx, vy),
            0xE => Instruction::I8XYE(vx, vy),
            _ => panic!(),
        },
        0x9000 => Instruction::I9XY0(vx, vy),
        0xA000 => Instruction::IANNN(tribble),
        0xB000 => Instruction::IBNNN(tribble),
        0xC000 => Instruction::ICXKK(vx, byte),
        0xD000 => Instruction::IDXYN(vx, vy, nibble),
        0xE000 => match op_code & 0x00FF {
            0x9E => Instruction::IEX9E(vx),
            0xA1 => Instruction::IEXA1(vx),
            _ => panic!(),
        }
        0xF000 => match op_code & 0x00FF {
            0x07 => Instruction::IFX07(vx),
            0x0A => Instruction::IFX0A(vx),
            0x15 => Instruction::IFX15(vx),
            0x18 => Instruction::IFX18(vx),
            0x1E => Instruction::IFX1E(vx),
            0x29 => Instruction::IFX29(vx),
            0x33 => Instruction::IFX33(vx),
            0x55 => Instruction::IFX55(vx),
            0x65 => Instruction::IFX65(vx),
            _ => panic!(),
        }
        _ => panic!(),
    }
}

fn execute(instruction: Instruction) {
    match instruction {
        Instruction::I00E0() => todo!(),
        Instruction::I00EE() => todo!(),
        Instruction::I0NNN(_) => todo!(),
        Instruction::I1NNN(_) => todo!(),
        Instruction::I2NNN(_) => todo!(),
        Instruction::I3XKK(_, _) => todo!(),
        Instruction::I4XKK(_, _) => todo!(),
        Instruction::I5XY0(_, _) => todo!(),
        Instruction::I6XKK(_, _) => todo!(),
        Instruction::I7XKK(_, _) => todo!(),
        Instruction::I8XY0(_, _) => todo!(),
        Instruction::I8XY1(_, _) => todo!(),
        Instruction::I8XY2(_, _) => todo!(),
        Instruction::I8XY3(_, _) => todo!(),
        Instruction::I8XY4(_, _) => todo!(),
        Instruction::I8XY5(_, _) => todo!(),
        Instruction::I8XY6(_, _) => todo!(),
        Instruction::I8XY7(_, _) => todo!(),
        Instruction::I8XYE(_, _) => todo!(),
        Instruction::I9XY0(_, _) => todo!(),
        Instruction::IANNN(_) => todo!(),
        Instruction::IBNNN(_) => todo!(),
        Instruction::ICXKK(_, _) => todo!(),
        Instruction::IDXYN(_, _, _) => todo!(),
        Instruction::IEX9E(_) => todo!(),
        Instruction::IEXA1(_) => todo!(),
        Instruction::IFX07(_) => todo!(),
        Instruction::IFX0A(_) => todo!(),
        Instruction::IFX15(_) => todo!(),
        Instruction::IFX18(_) => todo!(),
        Instruction::IFX1E(_) => todo!(),
        Instruction::IFX29(_) => todo!(),
        Instruction::IFX33(_) => todo!(),
        Instruction::IFX55(_) => todo!(),
        Instruction::IFX65(_) => todo!(),
    };
}

enum Instruction {
    // 00E0 - CLS
    // Clear the display
    I00E0(),

    // Return from a subroutine
    I00EE(),

    // (panic!) Jump to machine code routine at NNN
    I0NNN(u16),

    // Jump to location NNN
    I1NNN(u16),

    // Call subroutine at NNN
    I2NNN(u16),

    // Skip next instuction if VX = KK
    I3XKK(usize, u8),

    // Skip next instruction if VX != KK
    I4XKK(usize, u8),

    // Skip next instuction if VX = VY
    I5XY0(usize, usize),

    // (LD) Set VX = KK
    I6XKK(usize, u8),

    // (ADD) Set VX = KK
    I7XKK(usize, u8),

    // (LD) Set VX = VY
    I8XY0(usize, usize),

    // (OR) Set VX = VX OR VY
    I8XY1(usize, usize),

    // (AND) Set VX = VX AND VY
    I8XY2(usize, usize),

    // (XOR) Set VX = VX XOR VY
    I8XY3(usize, usize),

    // (ADD) Set VX = VX + VY, set VF = carry
    // If VX + VY is greater than 8 bits, then set VF to 1, otherwise 0. Keep lowest 8 bit as result.
    I8XY4(usize, usize),

    // (SUB) Set VX = VX - VY, set VF = NOT borrow
    // If VX > VY, then set VF to 1, otherwise 0. Subtract afterwards.
    I8XY5(usize, usize),

    // (SHR) Set VX = VX SHR 1
    // If the least-significant bit of VX is 1, then  set VF to 1, otherwise 0. Shift afterwards.
    I8XY6(usize, usize),

    // (SUBN) Set VX = VY - VX, set VF = NOT borrow
    // If VY > VX, then set VF to 1, otherwise 0. Subtract afterwards.
    I8XY7(usize, usize),

    // (SHL)
    I8XYE(usize, usize),
    I9XY0(usize, usize),
    IANNN(u16),
    IBNNN(u16),
    ICXKK(usize, u8),
    IDXYN(usize, usize, u8),
    IEX9E(usize),
    IEXA1(usize),
    IFX07(usize),
    IFX0A(usize),
    IFX15(usize),
    IFX18(usize),
    IFX1E(usize),
    IFX29(usize),
    IFX33(usize),
    IFX55(usize),
    IFX65(usize),
}
