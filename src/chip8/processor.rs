use super::constants::*;
use super::execution::*;
use super::loader::Loader;
use super::utils::*;
use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use std::os::windows::process;
use std::sync::*;
use std::time::Instant;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;
const BUFFER_CHUNK_SIZE: usize = 4;

#[derive(PartialEq, Clone, Copy)]
enum PixelState {
    Off,
    On,
}
use PixelState::*;

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
    pub stack: Vec<u16>,
    pub pixels: Option<Pixels>,
    pub last_execution: Instant,
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
            pixels: None,
            last_execution: Instant::now(),
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
        self.PC += 2;
        println!("PC: {:?}", self.PC);
        println!("I: {:?}", self.I);
        println!("V_REGS: {:?}\n", self.V_REGS);
    }

    pub fn draw_sprite(&mut self, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let height = Utils::resolve_hex(&[nibbles[3]]);
        let x = self.V_REGS[register_x as usize] % (REAL_WIDTH as u8);
        let y = self.V_REGS[register_y as usize] % (REAL_HEIGHT as u8);
        let base_address = self.I;

        self.VF = 0;
        for i in 0..height {
            let sprite_byte = *self
                .memory
                .get((base_address + i) as usize)
                .expect("Trying to access out-of-bounds memory");

            for j in 0..8 {
                let sprite_state = sprite_byte & (1 << j);
                let pixel_state = self.get_pixel((x + j) as usize, (y + (i as u8)) as usize) as u8;
                if sprite_state == 1 && pixel_state == 1 {
                    self.VF = 1;
                }
                let final_state = if sprite_state ^ pixel_state == 1 { On } else { Off };
                self.set_pixel((x + j) as usize, (y + (i as u8)) as usize, final_state);

            }
        }

        self.pixels.as_mut().unwrap().render();
    }




    pub fn get_pixel(&self, x: usize, y: usize) -> PixelState {
        let pixel = self
            .pixels.as_ref().unwrap()
            .frame()
            .chunks_exact(BUFFER_CHUNK_SIZE)
            .nth(y * REAL_WIDTH + x)
            .unwrap();

        if pixel[0] == 0xff {
            On
        } else {
            Off
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: PixelState) {
        let pixel = self
            .pixels.as_mut().unwrap()
            .frame_mut()
            .chunks_exact_mut(BUFFER_CHUNK_SIZE)
            .nth(y * REAL_WIDTH + x)
            .unwrap();
        let color_state = state as u8;
        pixel[0] = 0xff * color_state;
        pixel[1] = 0xff * color_state;
        pixel[2] = 0xff * color_state;
        pixel[3] = 0xff;
    }

    pub fn clear_screen(&mut self) {
        for pixel in self.pixels.as_mut().unwrap().frame_mut().chunks_exact_mut(BUFFER_CHUNK_SIZE) {
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        }
        self.pixels.as_mut().unwrap().render();
    }
}
