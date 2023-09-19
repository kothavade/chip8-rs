use chip8_backend::Chip8;
use sdl2::{
    audio::{AudioCallback, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
};
use std::time::Duration;

const CYCLES_PER_FRAME: u32 = 10;
const FPS: u32 = 60;
const KEYMAP: &[Keycode; 16] = COLEMAK_DH;

fn main() {
    let mut emu = Chip8::new();

    let rom = std::fs::read(std::env::args().nth(1).expect("No ROM provided"))
        .expect("Failed to read ROM");

    emu.load_rom(&rom);

    let sdl_context = sdl2::init().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let spec = AudioSpecDesired {
        freq: Some(3000),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        })
        .unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip8 Emulator", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = key_to_input(keycode) {
                        emu.keypress(key, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = key_to_input(keycode) {
                        emu.keypress(key, false);
                    }
                }
                _ => {}
            }
        }

        for _ in 0..CYCLES_PER_FRAME {
            emu.cycle();
        }
        emu.cycle_timer();
        draw(&mut canvas, &emu.get_display());
        play(&device, emu.get_sound());

        print!("\x1B[2J\x1B[1;1H");
        println!("{}", emu);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase < 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn play(device: &sdl2::audio::AudioDevice<SquareWave>, play: bool) {
    if play {
        device.resume();
    } else {
        device.pause();
    }
}

fn draw(canvas: &mut Canvas<Window>, display: &[bool; 64 * 32]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, &pixel) in display.iter().enumerate() {
        let x = (i % 64) as i32 * 10;
        let y = (i / 64) as i32 * 10;
        if pixel {
            canvas
                .fill_rect(Rect::new(x, y, 10, 10))
                .expect("Failed to draw rect");
        }
    }
}

fn key_to_input(keycode: Keycode) -> Option<u8> {
    KEYMAP.iter().position(|&k| k == keycode).map(|i| i as u8)
}

const COLEMAK_DH: &[Keycode; 16] = &[
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Num4,
    Keycode::Q,
    Keycode::W,
    Keycode::F,
    Keycode::P,
    Keycode::A,
    Keycode::R,
    Keycode::S,
    Keycode::T,
    Keycode::X,
    Keycode::C,
    Keycode::D,
    Keycode::V,
];

const QWERTY: &[Keycode; 16] = &[
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Num4,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::R,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::F,
    Keycode::Z,
    Keycode::X,
    Keycode::C,
    Keycode::V,
];
