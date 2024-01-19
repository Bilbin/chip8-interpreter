use super::constants::*;
use super::execution::*;
use super::graphics::*;
use pixels::Pixels;

#[allow(non_snake_case)]
pub struct Processor {
    pub PC: usize, // Program counter
    pub I: u16,    // Index register
    pub V_REGS: [u8; 15],

    pub VF: u8, // Flag register

    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8; 4096],
    pub stack: Vec<u16>,
    pub graphics: Graphics,
}

impl Processor {
    pub fn new() -> Self {
        let mut processor = Self {
            PC: ROM_START,
            I: 0,
            V_REGS: [0; 15],
            VF: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: Vec::new(),
            graphics: Graphics::new(),
        };

        // Load font into memory
        for i in 0..FONT.len() {
            processor.memory[FONT_START + i] = FONT[i];
        }

        processor
    }

    fn start(self) {
        self.graphics.start();
    }

    fn execute(&mut self) {
        let byte1 = self.memory[self.PC];
        let byte2 = self.memory[self.PC + 1];
        let nibbles = [byte1 >> 4, byte1 & 0b1111, byte2 >> 4, byte2 & 0b1111]
            .map(|nibble| format!("{:X}", nibble).chars().next().unwrap());

        InstructionHandler::execute(self, nibbles);
        self.PC += 2;
    }
}
