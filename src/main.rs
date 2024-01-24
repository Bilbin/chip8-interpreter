use chip8::{loader::Loader, processor::Processor};
use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use std::thread;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
mod chip8;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;

fn main() {
    let mut processor = Processor::new();
    Loader::load_rom(&mut processor, "roms/IBM Logo.ch8");
    //processor.start();
    println!("{:?}", processor.memory);

    let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Chip8 Interpreter")
        .with_inner_size(size)
        .with_max_inner_size(size)
        .with_min_inner_size(size);
    let window = window_builder.build(&event_loop).unwrap();
    let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
    let mut pixels = Pixels::new(REAL_WIDTH as u32, REAL_HEIGHT as u32, surface_texture)
        .unwrap();
    pixels
        .resize_buffer(REAL_WIDTH as u32, REAL_HEIGHT as u32)
        .unwrap();
    pixels.clear_color(Color::BLACK);
    pixels.render().unwrap();

    processor.pixels = Some(pixels);

    thread::spawn(move || {
        loop {
            // 700 instructions per second
            if processor.last_execution.elapsed().as_millis() >= 1000 {//(1000 / 700) {
                processor.execute();
                processor.last_execution = Instant::now();
            }
        }
    });

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
