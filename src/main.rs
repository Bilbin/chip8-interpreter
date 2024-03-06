mod chip8;
use chip8::{loader::Loader, processor::Processor, square::SquareWave};
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use rodio::{Sink, Source};
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

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
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("You must provide a rom as the first argument. Exiting...");
        return;
    }

    let rom_path = &args[0];

    let pressed_keys = Arc::new(Mutex::new([false; 16]));
    let shared_pressed_keys = Arc::clone(&pressed_keys);

    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("Unable to get audio output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let wave = SquareWave::new(240.0, 44100);
    let source = wave
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20)
        .repeat_infinite();
    sink.append(source);
    sink.pause();

    let mut processor = Processor::new(pressed_keys);
    Loader::load_rom(&mut processor, rom_path);

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

            if processor.sound_timer > 0 {
                sink.play();
            } else {
                sink.pause();
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
                if event == WindowEvent::CloseRequested {
                    elwt.exit();
                }
            }
        })
        .unwrap();
}
