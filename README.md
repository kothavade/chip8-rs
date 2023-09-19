# Chip8-rs

An emulator for the Chip-8, written in Rust.

## Run

```sh
nix develop # or direnv allow
# run desktop build with sdl
cd chip8-sdl
cargo run ../roms/games/Space\ Invaders\ \[David\ Winter\].ch8
# wip: run wasm build
cd chip8-wasm
wasm-pack build
```

## Tech

The desktop build uses SDL2 for windowing/graphics, input, and sound.
The work in progress WebAssembly build uses an HTML canvas and browser primitives.
