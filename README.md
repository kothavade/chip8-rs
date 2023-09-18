# Chip8-rs

An emulator for the Chip-8, written in Rust.

## Run

```sh
nix develop # or direnv allow
cargo run --release roms/games/Space\ Invaders\ \[David\ Winter\].ch8
```

## Tech

This uses SDL2 for windowing/graphics, input, and sound.
