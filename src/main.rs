mod cpu;
mod display;
mod input;
mod mem;

use cpu::Cpu;
use display::Display;
use input::Input;
use mem::Memory;
use sdl2::{event::Event, pixels::Color};
use std::time::Duration;

fn main() {
    let cpu = Cpu::default();
    let mut mem = Memory::default();
    let mut display = Display::default();
    let mut input = Input::default();

    let rom = std::fs::read("roms/invaders.ch8").expect("Unable to read rom");
    // let rom = std::fs::read(std::env::args().nth(1).unwrap()).expect("Unable to read rom");

    mem.load_rom(&rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip8 Emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut first = true;

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
                    println!("Key pressed: {:?}", keycode);
                    input.handle_key(keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    println!("Key let go: {:?}", keycode);
                    input.handle_key(keycode, false);
                }
                _ => {}
            }
        }
        if first {
            display.draw_sprite(
                0,
                0,
                &[0b11111111, 0b10000001, 0b10000001, 0b10000001, 0b11111111],
            );

            display.draw_sprite(
                10,
                10,
                &[
            first = false;
        }
        display.draw_to_canvas(&mut canvas);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
