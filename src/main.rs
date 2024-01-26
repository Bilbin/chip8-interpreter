use chip8::{loader::Loader, processor::Processor};
use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod chip8;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;

// Original COSMAC VIP keypad looks like:
// 1   2   3   C
// 4   5   6   D
// 7   8   9   E
// A   0   B   F
// This mapping is made such that given a key value 0-F,
// you can easily get the corresponding keyboard keycode
const KEY_BINDINGS: [KeyCode; 16] = [
    KeyCode::KeyX,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::KeyQ,
    KeyCode::KeyW,
    KeyCode::KeyE,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::KeyZ,
    KeyCode::KeyC,
    KeyCode::Digit4,
    KeyCode::KeyR,
    KeyCode::KeyF,
    KeyCode::KeyV,
];

fn main() {
    let pressed_keys = Arc::new(Mutex::new([false; 16]));
    let shared_pressed_keys = Arc::clone(&pressed_keys);

    let mut processor = Processor::new(pressed_keys);
    Loader::load_rom(&mut processor, "roms/Cave.ch8");
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
    let mut pixels = Pixels::new(REAL_WIDTH as u32, REAL_HEIGHT as u32, surface_texture).unwrap();
    pixels
        .resize_buffer(REAL_WIDTH as u32, REAL_HEIGHT as u32)
        .unwrap();
    pixels.clear_color(Color::BLACK);
    pixels.render().unwrap();

    processor.pixels = Some(pixels);
    let mut last_timer_tick = Instant::now();

    thread::spawn(move || {
        loop {
            // 700 instructions per second
            if processor.last_execution.elapsed().as_millis() >= (1000 / 700) {
                processor.execute();
                processor.last_execution = Instant::now();
            }

            // Update timers 60 times a second
            if last_timer_tick.elapsed().as_millis() >= (1000 / 60) {
                processor.delay_timer = if processor.delay_timer > 0 {
                    processor.delay_timer - 1
                } else {
                    0
                };
                processor.sound_timer = if processor.sound_timer > 0 {
                    processor.sound_timer - 1
                } else {
                    0
                };
                last_timer_tick = Instant::now();
            }
        }
    });

    let mut input = WinitInputHelper::new();

    event_loop
        .run(move |event, elwt| {
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
        })
        .unwrap();
}
