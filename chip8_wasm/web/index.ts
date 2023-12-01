import init, { Chip8Wasm } from "../pkg/chip8_wasm.js";

init().then(() => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const SCALE = 15;
  let ticks_per_frame = 10;
  let animationFrameId: number | null = null;
  let lastFrameTime: number = 0;

  const canvas = document.getElementById("canvas") as HTMLCanvasElement;

  canvas.width = WIDTH * SCALE;
  canvas.height = HEIGHT * SCALE;

  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

  const input = document.getElementById("fileinput") as HTMLInputElement;

  const chip8 = new Chip8Wasm();

  document.addEventListener("keydown", (evt) => {
    chip8.keypress(evt, true);
  });

  document.addEventListener("keyup", (evt) => {
    chip8.keypress(evt, false);
  });

  input.addEventListener(
    "change",
    () => {
      if (animationFrameId !== null) {
        window.cancelAnimationFrame(animationFrameId);
      }
      let file = input.files![0];
      if (!file) {
        alert("Failed to read file");
        input.value = "";
        // reset canvas
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
        return;
      }
      // Load in game as Uint8Array, send to .wasm, start main loop
      let fr = new FileReader();
      fr.onload = () => {
        const rom = new Uint8Array(fr.result as ArrayBuffer);
        chip8.reset();
        chip8.load_rom(rom);
        startLoop(chip8);
      };
      fr.readAsArrayBuffer(file);
    },
    false
  );

  const loop = (chip8: Chip8Wasm, timestamp: number) => {
    const deltaTime = timestamp - lastFrameTime;
    lastFrameTime = timestamp;

    for (let i = 0; i < ticks_per_frame; i++) {
      chip8.cycle();
    }
    chip8.cycle_timer();

    ctx.clearRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
    ctx.fillStyle = "white";
    chip8.draw(SCALE);

    animationFrameId = window.requestAnimationFrame((timestamp) => {
      loop(chip8, timestamp);
    });
  };

  const startLoop = (chip8: Chip8Wasm) => {
    if (animationFrameId !== null) {
      window.cancelAnimationFrame(animationFrameId);
    }
    lastFrameTime = performance.now();
    loop(chip8, lastFrameTime);
  };
});
