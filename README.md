# Chip8-rs

An emulator for the Chip-8, written in Rust.

## Demo

https://github.com/kothavade/chip8-rs/assets/60118973/82bdbd61-c501-43e1-958c-42e510c820cb

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
