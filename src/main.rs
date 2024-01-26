use chip8::{loader::Loader, processor::Processor};
use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use std::thread;
use std::time::Instant;
use winit_input_helper::WinitInputHelper;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowBuilder,
    event::{Event, WindowEvent},
    keyboard::KeyCode,
};
use std::sync::{Arc, Mutex};

mod chip8;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;
const KEY_BINDINGS: [KeyCode; 16] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::KeyQ,
    KeyCode::KeyW,
    KeyCode::KeyE,
    KeyCode::KeyR,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::KeyF,
    KeyCode::KeyZ,
    KeyCode::KeyX,
    KeyCode::KeyC,
    KeyCode::KeyV,
];

fn main() {
    let pressed_keys = Arc::new(Mutex::new([false; 16]));
    let shared_pressed_keys = Arc::clone(&pressed_keys);

    let mut processor = Processor::new(pressed_keys);
    Loader::load_rom(&mut processor, "roms/IBM Logo.ch8");
    //processor.start();
    println!("{:?}", processor.memory);

    let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let event_loop = EventLoop::new().unwrap();
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
            if processor.last_execution.elapsed().as_millis() >= (1000 / 1) {
                processor.execute();
                processor.last_execution = Instant::now();
            }
        }
    });


    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, elwt| {
        if input.update(&event) {
            for (ind, i) in KEY_BINDINGS.iter().enumerate() {
                shared_pressed_keys.lock().unwrap()[ind] = input.key_held(*i);
            }
        }
        
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => (),
            }
        }
    }).unwrap();
}
