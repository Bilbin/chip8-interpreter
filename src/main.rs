use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;

mod chip8;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;
const BUFFER_CHUNK_SIZE: usize = 4;

#[derive(PartialEq, Clone, Copy)]
enum PixelState {
    OFF,
    ON,
}
use PixelState::*;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Chip8 Interpreter")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let window = window_builder.build(&event_loop).unwrap();
    let size = window.inner_size();
    
    let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();
    pixels.resize_buffer(REAL_WIDTH as u32, REAL_HEIGHT as u32).unwrap();
    pixels.clear_color(Color::BLACK);
    pixels.render().unwrap();

    modify_pixel(&mut pixels, 0, 0, ON);
    pixels.render().unwrap();

    modify_pixel(&mut pixels, 20, 20, ON);
    pixels.render().unwrap();

    event_loop.run(move |event, _, control_flow| {
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

fn modify_pixel(screen: &mut Pixels, x: usize, y: usize, state: PixelState) {
    let pixel = screen.frame_mut()
        .chunks_exact_mut(BUFFER_CHUNK_SIZE)
        .nth(y * REAL_WIDTH + x)
        .unwrap();
    let color_state = if state == ON { 1 } else { 0 };
    pixel[0] = 0xff * color_state;
    pixel[1] = 0xff * color_state;
    pixel[2] = 0xff * color_state;
    pixel[3] = 0xff;
}
