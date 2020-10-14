import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#cccccc";
const DEAD_COLOR = "#ffffff";
const ALIVE_COLOR = "#000000";

// Construct the Game of Life universe.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border around each.
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext("2d");

// Call `Universe::tick` and then draw the current universe to `<canvas>`
// at each iteration.
const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

// Draw the universe grid as a series of vertical and horizontal lines.
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    const max_x = (CELL_SIZE + 1) * width + 1;
    const max_y = (CELL_SIZE + 1) * height + 1;

    for (let x = 0; x <= width; ++x) {
        const curr_x = x * (CELL_SIZE + 1) + 1;
        ctx.moveTo(curr_x,   0  );
        ctx.lineTo(curr_x, max_y);
    }

    for (let y = 0; y <= height; ++y) {
        const curr_y = y * (CELL_SIZE + 1) + 1;
        ctx.moveTo(  0,   curr_y);
        ctx.lineTo(max_x, curr_y);
    }
}

// Draw the universe cells as white or black rectangles depending on whether
// the cell is dead or alive, respectively.
//
// @Note: to draw them, we get a pointer to the `cells` buffer, construct a
// Uint8Array overlaying it and iterate over each cell. By working with pointers
// and overlays, we avoid copying the cells across the boundary on every tick,
// as we can directly access WebAssembly's linear memory via `memory`.
const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            // @Todo: export `get_index` from lib.rs.
            const idx = row * width + col;

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

// Draw the initial state of the universe (i.e. the first "tick")
// and start the rendering process.
drawGrid();
drawCells();

requestAnimationFrame(renderLoop);
