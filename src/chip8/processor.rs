use pixels::Pixels;
use super::constants::*;
use super::execution::*;

#[allow(non_snake_case)]
pub struct Processor {
    pub PC: usize, // Program counter
    pub I: u16,    // Index register
    pub V0: u8,    // General registers
    pub V1: u8,
    pub V2: u8,
    pub V3: u8,
    pub V4: u8,
    pub V5: u8,
    pub V6: u8,
    pub V7: u8,
    pub V8: u8,
    pub V9: u8,
    pub VA: u8,
    pub VB: u8,
    pub VC: u8,
    pub VD: u8,
    pub VE: u8,

    pub VF: u8, // Flag register

    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8; 4096],
    pub stack: Vec<u16>,
    pub instruction_handler: InstructionHandler,
    pub pixels: Pixels,
}

impl Processor {
    pub 
    fn new(pixels: Pixels) -> Self {
        let instruction_handler = InstructionHandler::new();
        let mut instance = Self {
            PC: 0x200,
            I: 0,
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: Vec::new(),
            pixels,
        };

        // Load font into memory
        for i in 0..FONT.len() {
            instance.memory[FONT_START + i] = FONT[i];
        }

        instance
    }

    fn execute(&mut self) {
        let byte1 = self.memory[self.PC];
        let byte2 = self.memory[self.PC + 1];
        let nibbles = [byte1 >> 4, byte1 & 0b1111, byte2 >> 4, byte2 & 0b1111]
            .map(|nibble| format!("{:X}", nibble).chars().next().unwrap());

        InstructionHandler::execute(self, nibbles);
    }
}
