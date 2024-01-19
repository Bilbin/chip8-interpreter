use super::processor::*;
use super::utils::*;
use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

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

pub struct Graphics {
    pixels: Pixels,
    event_loop: EventLoop<()>,
}

impl Graphics {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title("Chip8 Interpreter")
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let window = window_builder.build(&event_loop).unwrap();
        let size = window.inner_size();

        let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
        let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();
        pixels
            .resize_buffer(REAL_WIDTH as u32, REAL_HEIGHT as u32)
            .unwrap();
        pixels.clear_color(Color::BLACK);
        pixels.render().unwrap();

        Self { pixels, event_loop }
    }

    pub fn start(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            }
        });
    }

    pub fn draw_sprite(&mut self, processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let height = Utils::resolve_hex(&[nibbles[3]]);
        let x = processor.V_REGS[register_x as usize] % (REAL_WIDTH as u8);
        let y = processor.V_REGS[register_y as usize] % (REAL_HEIGHT as u8);
        let base_address = processor.I;

        processor.VF = 0;
        for i in 0..height {
            let sprite_byte = processor
                .memory
                .get((base_address + i) as usize)
                .expect("Trying to access out-of-bounds memory");

            for j in 0..8 {
                let sprite_state = sprite_byte & (1 << j);
                let pixel_state = self.get_pixel((x + j) as usize, (y + (i as u8)) as usize) as u8;
                if sprite_state == 1 && pixel_state == 1 {
                    processor.VF = 1;
                }
                let final_state = if sprite_state ^ pixel_state == 1 { On } else { Off };
                self.set_pixel((x + j) as usize, (y + (i as u8)) as usize, final_state);

            }
        }

        self.pixels.render();
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> PixelState {
        let pixel = self
            .pixels
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
            .pixels
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
        for pixel in self.pixels.frame_mut().chunks_exact_mut(BUFFER_CHUNK_SIZE) {
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        }
    }
}
