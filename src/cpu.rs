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

        // fetch instruction
        // decode instruction
        // execute instuction

        // update timers
    }
}

fn oof() {
    println!("oof");
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