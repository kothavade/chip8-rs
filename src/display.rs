use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Debug)]
pub struct Display {
    pub display: [bool; 64 * 32],
}

impl Default for Display {
    fn default() -> Self {
        Self {
            display: [false; 64 * 32],
        }
    }
}

impl Display {
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) {
        for (i, byte) in sprite.iter().enumerate() {
            for j in 0..8 {
                let index = (x as usize + j + ((y as usize + i) * 64)) % 2048;
                let pixel = self.display[index];
                let pixel = pixel ^ (byte >> (7 - j) & 1 == 1);
                self.display[index] = pixel;
            }
        }
    }
    pub fn draw_to_canvas(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // dray the diplay array to Canvas
        // each pixel in the array corresponds to a 10x10 rect
        // 64x32 = 640x320
        for (i, pixel) in self.display.iter().enumerate() {
            let x = (i % 64) as i32 * 10;
            let y = (i / 64) as i32 * 10;
            if *pixel {
                canvas
                    .fill_rect(Rect::new(x, y, 10, 10))
                    .expect("Failed to draw rect");
            }
        }
        canvas.present();
    }
}
