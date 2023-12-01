mod utils;
use chip8_backend::Chip8;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

const KEYMAP: &[&str; 16] = QWERTY;

#[wasm_bindgen]
pub struct Chip8Wasm {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        Chip8Wasm {
            chip8: Chip8::new(),
            // most ergonomic rust library
            ctx: web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id("canvas"))
                .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
                .and_then(|canvas| canvas.get_context("2d").ok())
                .unwrap()
                .and_then(|context| context.dyn_into::<CanvasRenderingContext2d>().ok())
                .unwrap_or_else(|| panic!("Failed to initialize CanvasRenderingContext2d")),
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: Uint8Array) {
        self.chip8.load_rom(&rom.to_vec());
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        if let Some(k) = key_to_input(&evt.key()) {
            self.chip8.keypress(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        self.chip8.cycle();
    }

    #[wasm_bindgen]
    pub fn cycle_timer(&mut self) {
        self.chip8.cycle_timer();
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, scale: usize) {
        for (i, &pixel) in self.chip8.get_display().iter().enumerate() {
            let x = (i % 64) as f64 * scale as f64;
            let y = (i / 64) as f64 * scale as f64;
            if pixel {
                self.ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
                self.ctx.fill_rect(x, y, scale as f64, scale as f64);
            } else {
                // self.ctx.clear_rect(x, y, scale as f64, scale as f64);
                self.ctx.set_fill_style(&JsValue::from_str("#000000"));
                self.ctx.fill_rect(x, y, scale as f64, scale as f64);
            }
        }
    }
}

fn key_to_input(key: &str) -> Option<u8> {
    KEYMAP.iter().position(|&k| k == key).map(|i| i as u8)
}

const COLEMAK_DH: &[&str; 16] = &[
    "1", "2", "3", "4", "q", "w", "f", "p", "a", "r", "s", "t", "x", "c", "d", "v",
];

const QWERTY: &[&str; 16] = &[
    "1", "2", "3", "4", "q", "w", "e", "r", "a", "s", "d", "f", "z", "x", "c", "v",
];
