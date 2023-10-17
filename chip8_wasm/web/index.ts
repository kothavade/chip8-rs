import { Chip8Wasm } from "../pkg/chip8_wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;
const TICKS_PER_FRAME = 10;

let frame = 0;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;

canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const input = document.getElementById("fileinput") as HTMLInputElement;

const chip8 = new Chip8Wasm();

document.addEventListener("keydown", function (evt) {
  chip8.keypress(evt, true);
});

document.addEventListener("keyup", function (evt) {
  chip8.keypress(evt, false);
});

input.addEventListener(
  "change",
  () => {
    if (frame != 0) {
      window.cancelAnimationFrame(frame);
    }
    let file = input.files![0];
    if (!file) {
      alert("Failed to read file");
      return;
    }
    // Load in game as Uint8Array, send to .wasm, start main loop
    let fr = new FileReader();
    fr.onload = () => {
      const rom = new Uint8Array(fr.result as ArrayBuffer);
      chip8.reset();
      chip8.load_rom(rom);
      loop(chip8);
    };
    fr.readAsArrayBuffer(file);
  },
  false
);

const loop = (chip8: Chip8Wasm) => {
  for (let i = 0; i < TICKS_PER_FRAME; i++) {
    chip8.cycle();
  }
  chip8.cycle_timer();
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
  ctx.fillStyle = "white";
  chip8.draw(SCALE);
  frame = window.requestAnimationFrame(() => {
    loop(chip8);
  });
};
