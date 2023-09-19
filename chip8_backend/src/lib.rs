use std::fmt::Display;

pub struct Chip8 {
    // CPU:
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

    // Memory:
    // 4k memory: 0x000 - 0xFFF
    // 0x000 - 0x1FF: Original Chip-8 interpreter
    // 0x050 - 0x0A0: Font set
    // 0x200 - 0xFFF: Program ROM and work RAM
    ram: [u8; 4096],

    // Stack
    // Stores 16 16-bit values, only need 1-2 values
    stack: [u16; 16],

    // Display
    // 64x32 pixels
    display: [bool; 64 * 32],

    // Input
    // 16 keys
    keys: [bool; 16],
}

// Public methods
impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0; 4096];
        // Font is copied into memory starting at 0x50
        ram[0x50..(0x50 + FONT_SIZE)].copy_from_slice(&FONT);
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            ram,
            stack: [0; 16],
            display: [false; 64 * 32],
            keys: [false; 16],
        }
    }

    pub fn reset(&mut self) {
        self.v = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.ram = [0; 4096];
        self.ram[0x50..(0x50 + FONT_SIZE)].copy_from_slice(&FONT);
        self.stack = [0; 16];
        self.display = [false; 64 * 32];
        self.keys = [false; 16];
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        // Load ROM into memory starting at 0x200
        self.ram[0x200..(0x200 + rom.len())].copy_from_slice(rom);
    }

    pub fn keypress(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize] = pressed;
    }

    pub fn get_display(&self) -> &[bool; 64 * 32] {
        &self.display
    }

    pub fn get_sound(&self) -> bool {
        self.st > 0
    }

    pub fn cycle(&mut self) {
        // Fetch opcode
        let opcode = self.fetch_opcode();
        self.execute(opcode);
    }

    pub fn cycle_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}

// Private methods
impl Chip8 {
    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn next(&mut self) {
        self.pc += 2;
    }

    fn skip_if(&mut self, condition: bool) {
        if condition {
            self.pc += 4;
        } else {
            self.next();
        }
    }

    fn fetch_opcode(&self) -> u16 {
        // Fetch opcode from memory
        // Opcodes are 2 bytes long
        // Memory is 8-bit, so we need to combine two bytes
        // Shift first byte left by 8 bits and combine with second byte
        let pc = self.pc as usize;
        let byte1 = self.ram[pc] as u16;
        let byte2 = self.ram[pc + 1] as u16;
        byte1 << 8 | byte2
    }

    fn random_byte(&self) -> u8 {
        rand::random::<u8>()
    }

    fn execute(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u16;

        match nibbles {
            (0x0, 0x0, 0x0, 0x0) => return,
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x0, _, _, _) => self.op_0nnn(nnn),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xye(x),
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nnn),
            (0xC, _, _, _) => self.op_cxkk(x, kk),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(x),
            (0xE, _, 0xA, 0x1) => self.op_exa1(x),
            (0xF, _, 0x0, 0x7) => self.op_fx07(x),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(x),
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.op_fx29(x),
            (0xF, _, 0x3, 0x3) => self.op_fx33(x),
            (0xF, _, 0x5, 0x5) => self.op_fx55(x),
            (0xF, _, 0x6, 0x5) => self.op_fx65(x),
            _ => panic!("Unknown opcode: {:04X}", opcode),
        }
    }
}

// Opcodes
impl Chip8 {
    // Clear display
    fn op_00e0(&mut self) {
        self.display = [false; 64 * 32];
        self.next();
    }
    // Return from subroutine
    fn op_00ee(&mut self) {
        self.pc = self.pop();
    }
    // Jump to address nnn
    fn op_0nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }
    // Jump to address nnn
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }
    // Call subroutine at nnn
    fn op_2nnn(&mut self, nnn: u16) {
        self.push(self.pc + 2);
        self.pc = nnn;
    }
    // Skip next instruction if Vx == kk
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        self.skip_if(self.v[x] == kk);
    }
    // Skip next instruction if Vx != kk
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        self.skip_if(self.v[x] != kk);
    }
    // Skip next instruction if Vx == Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        self.skip_if(self.v[x] == self.v[y]);
    }
    // Set Vx = kk
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.next();
    }
    // Set Vx = Vx + kk
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
        self.next();
    }
    // Set Vx = Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.next();
    }
    // Set Vx = Vx OR Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.next();
    }
    // Set Vx = Vx AND Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.next();
    }
    // Set Vx = Vx XOR Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.next();
    }
    // Set Vx = Vx + Vy, set VF = carry
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = overflow as u8;
        self.next();
    }
    // Set Vx = Vx - Vy, set VF = NOT borrow
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = !overflow as u8;
        self.next();
    }
    // Set Vx = Vx SHR 1
    fn op_8xy6(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0x1;
        self.v[x] >>= 1;
        self.next();
    }
    // Set Vx = Vy - Vx, set VF = NOT borrow
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = result;
        self.v[0xF] = !overflow as u8;
        self.next();
    }
    // Set Vx = Vx SHL 1
    fn op_8xye(&mut self, x: usize) {
        self.v[0xF] = self.v[x] >> 7;
        self.v[x] <<= 1;
        self.next();
    }
    // Skip next instruction if Vx != Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        self.skip_if(self.v[x] != self.v[y]);
    }
    // Set I = nnn
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
        self.next();
    }
    // Jump to location nnn + V0
    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }
    // Set Vx = random byte AND kk
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.random_byte() & kk;
        self.next();
    }
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
    fn op_dxyn(&mut self, x: usize, y: usize, n: u16) {
        let vx = self.v[x];
        let vy = self.v[y];
        self.v[0xF] = 0;
        for byte in 0..n {
            let sprite = self.ram[self.i as usize + byte as usize];
            for bit in 0..8 {
                let x = (vx + bit) % 64;
                let y = (vy + byte as u8) % 32;
                let pixel = (sprite >> (7 - bit)) & 0x1;
                let index = x as usize + y as usize * 64;
                if pixel == 1 && self.display[index] {
                    self.v[0xF] = 1;
                }
                self.display[index] ^= pixel == 1;
            }
        }
        self.next();
    }
    // Skip next instruction if key with the value of Vx is pressed
    fn op_ex9e(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] {
            self.pc += 4;
        } else {
            self.next();
        }
    }
    // Skip next instruction if key with the value of Vx is not pressed
    fn op_exa1(&mut self, x: usize) {
        self.skip_if(!self.keys[self.v[x] as usize])
    }
    // Set Vx = delay timer value
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
        self.next();
    }
    // Wait for a key press, store the value of the key in Vx
    fn op_fx0a(&mut self, x: usize) {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                self.v[x] = i as u8;
                self.next();
                break;
            }
        }
    }
    // Set delay timer = Vx
    fn op_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
        self.next();
    }
    // Set sound timer = Vx
    fn op_fx18(&mut self, x: usize) {
        self.st = self.v[x];
        self.next();
    }
    // Set I = I + Vx
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as u16;
        self.next();
    }
    // Set I = location of sprite for digit Vx
    fn op_fx29(&mut self, x: usize) {
        self.i = self.v[x] as u16 * 5;
        self.next();
    }
    // Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn op_fx33(&mut self, x: usize) {
        let vx = self.v[x];
        self.ram[self.i as usize] = vx / 100;
        self.ram[self.i as usize + 1] = (vx / 10) % 10;
        self.ram[self.i as usize + 2] = vx % 10;
        self.next();
    }
    // Store registers V0 through Vx in memory starting at location I
    fn op_fx55(&mut self, x: usize) {
        for i in 0..=x {
            self.ram[self.i as usize + i as usize] = self.v[i as usize];
        }
        self.next();
    }
    // Read registers V0 through Vx from memory starting at location I
    fn op_fx65(&mut self, x: usize) {
        for i in 0..=x {
            self.v[i as usize] = self.ram[self.i as usize + i as usize];
        }
        self.next();
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("Chip-8 Emulator\n");
        s.push_str("---------------\n");
        s.push_str(&format!("PC: {:04X}\n", self.pc));
        s.push_str(&format!("I: {:04X}\n", self.i));
        s.push_str(&format!("SP: {:02X}\n", self.sp));
        s.push_str(&format!("DT: {:02X}\n", self.dt));
        s.push_str(&format!("ST: {:02X}\n", self.st));
        s.push_str(&format!("V: {:02X?}\n", self.v));
        s.push_str(&format!("Stack: {:04X?}\n", self.stack));
        // s.push_str(&format!("Display: {:02X?}\n", self.display));
        s.push_str(&format!("Keys: {:02X?}\n", self.keys));
        write!(f, "{}", s)
    }
}

const FONT_SIZE: usize = 80;
const FONT: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
