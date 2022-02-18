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
        oof();

        // fetch opcode
        // decode opcode
        // execute opcode

        // update timers
    }
}

fn oof() {
    println!("oof");
}

enum Instruction {
    // Clear the display
    Op00E0(),

    // Return from a subroutine
    Op00EE(),

    // (panic!) Jump to machine code routine at NNN
    Op0NNN(u16),

    // Jump to location NNN
    Op1NNN(u16),

    // Call subroutine at NNN
    Op2NNN(u16),

    // Skip next instuction if VX = KK
    Op3XKK(usize, u8),

    // Skip next instruction if VX != KK
    Op4XKK(usize, u8),

    // Skip next instuction if VX = VY
    Op5XY0(usize, usize),

    // (LD) Set VX = KK
    Op6XKK(usize, u8),

    // (ADD) Set VX = KK
    Op7XKK(usize, u8),

    // (LD) Set VX = VY
    Op8XY0(usize, usize),

    // (OR) Set VX = VX OR VY
    Op8XY1(usize, usize),

    // (AND) Set VX = VX AND VY
    Op8XY2(usize, usize),

    // (XOR) Set VX = VX XOR VY
    Op8XY3(usize, usize),

    // (ADD) Set VX = VX + VY, set VF = carry
    // If VX + VY is greater than 8 bits, then set VF to 1, otherwise 0. Keep lowest 8 bit as result.
    Op8XY4(usize, usize),

    // (SUB) Set VX = VX - VY, set VF = NOT borrow
    // If VX > VY, then set VF to 1, otherwise 0. Subtract afterwards.
    Op8XY5(usize, usize),

    // (SHR) Set VX = VX SHR 1
    // If the least-significant bit of VX is 1, then  set VF to 1, otherwise 0. Shift afterwards.
    Op8XY6(usize, usize),

    // (SUBN) Set VX = VY - VX, set VF = NOT borrow
    // If VY > VX, then set VF to 1, otherwise 0. Subtract afterwards.
    Op8XY7(usize, usize),

    // (SHL) 
    Op8XYE(),
    Op9XY0(),
    OpANNN(),
    OpBNNN(),
    OpCXKK(),
    OpDXYN(),
    OpEX9E(),
    OpEXA1(),
    OpFX07(),
    OpFX0A(),
    OpFX15(),
    OpFX18(),
    OpFX1E(),
    OpFX29(),
    OpFX33(),
    OpFX55(),
    OpFX65(),
}