use super::constants::*;
use super::execution::*;
use pixels::Pixels;
use std::time::Instant;
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Processor {
    pub PC: usize, // Program counter
    pub I: u16,    // Index register
    pub V_REGS: [u8; 15],

    pub VF: u8, // Flag register

    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8; 4096],
    pub stack: Vec<[char; 3]>,
    pub pixels: Option<Pixels>,
    pub last_execution: Instant,
    pub pressed_keys: Arc<Mutex<[bool; 16]>>,
}

impl Processor {
    pub fn new(pressed_keys: Arc<Mutex<[bool; 16]>>) -> Self {
        let mut processor = Self {
            PC: ROM_START,
            I: 0,
            V_REGS: [0; 15],
            VF: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: Vec::new(),
            pixels: None,
            last_execution: Instant::now(),
            pressed_keys,
        };

        // Load font into memory
        for i in 0..FONT.len() {
            processor.memory[FONT_START + i] = FONT[i];
        }

        processor
    }

    pub fn execute(&mut self) {
        let byte1 = self.memory[self.PC];
        let byte2 = self.memory[self.PC + 1];
        let nibbles = [byte1 >> 4, byte1 & 0b1111, byte2 >> 4, byte2 & 0b1111]
            .map(|nibble| format!("{:X}", nibble).chars().next().unwrap());
        
        println!("Executing: {:?}", nibbles);
        InstructionHandler::execute(self, nibbles);
        
        // Check that it wasn't a jump or subroutine return
        if nibbles[0] != '1' && nibbles[0] != '2' && nibbles[0] != 'B' {
            self.PC += 2;
        }

        // println!("PC: {:?}", self.PC);
        // println!("I: {:?}", self.I);
        // println!("V_REGS: {:?}\n", self.V_REGS);
    }
}
