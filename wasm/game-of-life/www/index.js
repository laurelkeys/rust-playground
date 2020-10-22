import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/game_of_life_bg";
import { fps } from "./fps.js";

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

// Keep track of the identifier returned by `requestAnimationFrame`.
let requestAnimationId = null;

// Check whether or not the game is paused.
const isPaused = () => {
    return requestAnimationId === null;
}

// Call `Universe::tick` and then draw the current universe to `<canvas>`
// at each iteration.
const renderLoop = () => {
    // debugger; // invoke debugging functionality if it's available
    fps.render();

    universe.tick();
    drawUniverse();

    requestAnimationId = requestAnimationFrame(renderLoop);
}

const playPauseButton = document.getElementById("play-pause");

// Resume the `renderLoop` animation.
const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

// Cancel the next animation frame.
const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(requestAnimationId);
    requestAnimationId = null;
};

// Draw the universe grid as a series of vertical and horizontal lines.
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    const max_x = (CELL_SIZE + 1) * width + 1;
    const max_y = (CELL_SIZE + 1) * height + 1;

    for (let x = 0; x <= width; ++x) {
        const curr_x = x * (CELL_SIZE + 1) + 1;
        ctx.moveTo(curr_x, 0);
        ctx.lineTo(curr_x, max_y);
    }

    for (let y = 0; y <= height; ++y) {
        const curr_y = y * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, curr_y);
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

    const fillCells = (cellState) => {
        for (let row = 0; row < height; ++row) {
            for (let col = 0; col < width; ++col) {
                const idx = universe.getIndex(row, col);
                if (cells[idx] == cellState) {
                    ctx.fillRect(
                        col * (CELL_SIZE + 1) + 1,
                        row * (CELL_SIZE + 1) + 1,
                        CELL_SIZE,
                        CELL_SIZE
                    );
                }
            }
        }
    }

    // Alive cells.
    ctx.fillStyle = ALIVE_COLOR;
    fillCells(Cell.Alive);

    // Dead cells.
    ctx.fillStyle = DEAD_COLOR;
    fillCells(Cell.Dead);

    ctx.stroke();
}

const drawUniverse = () => {
    drawGrid();
    drawCells();
}

// Add event listeners for `<canvas>` and `<button>`s.

canvas.addEventListener("click", event => {
    // Convert page-relative coordinates into canvas-relative coordinates.
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    // Convert canvas-relative coordinates into row and column
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    // Toggle the selected cell and redraw.
    universe.toggleCell(row, col);
    drawUniverse();
});

playPauseButton.addEventListener("click", _ => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

document.getElementById("randomize").addEventListener("click", _ => {
    universe.randomize();
    drawUniverse();
});

document.getElementById("exterminate").addEventListener("click", _ => {
    universe.exterminate();
    drawUniverse();
});

// Start the Game of Life!
play();
