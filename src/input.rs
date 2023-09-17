use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct Input<'a> {
    pub layout: Layout<'a>,
    keys: [bool; 16],
}

impl Input<'_> {
    pub fn handle_key(&mut self, key: Keycode, pressed: bool) {
        if let Some(index) = self.layout.0.iter().position(|&k| k == key) {
            self.keys[index] = pressed;
        }
    }
}

pub struct Layout<'a>(&'a [Keycode; 16]);

impl Default for Layout<'_> {
    fn default() -> Self {
        Layout(Self::COLEMAK_DH)
    }
}

impl Layout<'_> {
    pub const QWERTY: &[Keycode; 16] = QWERTY_KEY_MAP;
    pub const COLEMAK_DH: &[Keycode; 16] = COLEMAK_DH_KEY_MAP;
}

const COLEMAK_DH_KEY_MAP: &[Keycode; 16] = &[
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

const QWERTY_KEY_MAP: &[Keycode; 16] = &[
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
