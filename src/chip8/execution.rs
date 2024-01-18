use super::processor::*;
use pixels::Pixels;

pub struct InstructionHandler { 
    processor: Processor,
    pixels: Pixels,
}

impl InstructionHandler {
    pub fn new(processor: Processor) -> Self {
        Self {
            processor,
            pixels: processor.pixel,
        }
    }
    pub fn execute(&mut self, nibbles: [char; 4]) {
        self.cl
        match nibbles[0] {
            '0' => self.clear
        } 
    }
    pub fn clear_screen() {
        
    }
}