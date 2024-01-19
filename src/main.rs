use chip8::{processor::Processor, loader::Loader};
mod chip8;

fn main() {
    let mut processor = Processor::new();
    Loader::load_rom(&mut processor, "roms/IBM Logo.ch8");
    processor.start();
}
