use std::fs::File;
use std::io::Read;
use super::constants::ROM_START;
use super::processor::*;

pub struct Loader {}

impl Loader {
    pub fn load_rom(processor: &mut Processor, filename: &str) {
        let mut buffer = Vec::new();
        let mut file = File::open(filename).expect("Failed to open rom");
        file.read_to_end(&mut buffer);
        
        processor.memory[ROM_START..buffer.len() + ROM_START].copy_from_slice(&buffer)
    }
}