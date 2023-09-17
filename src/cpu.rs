#[derive(Debug, Default)]
pub struct Cpu {
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

impl Cpu {}
