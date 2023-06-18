#![allow(dead_code)]
pub struct Cpu {
    // 4k memory: 0x000 - 0xFFF
    // 0x000 - 0x1FF: Original Chip-8 interpreter
    // 0x050 - 0x0A0: Font set
    // 0x200 - 0xFFF: Program ROM and work RAM
    mem: [u8; 4096],

    // 16 8-bit general registers V0-VF
    // VF is used as a flag for some instructions
    v: [u8; 16],

    // 16-bit register to store memory addresses
    i: u16,

    // 16-bit program counter
    pc: u16,

    // 8-bit stack pointer
    sp: u8,

    // 8-bit delay timer
    dt: u8,

    // 8-bit sound timer
    st: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            mem: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }
}
