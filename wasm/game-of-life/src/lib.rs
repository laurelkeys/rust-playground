// @Todo: continue from https://rustwasm.github.io/docs/book/game-of-life/implementing.html#rendering-with-javascript

mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

/*
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/

// @Note: by using `repr(u8)` we ensure that each cell is represented as a single byte.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    /// Returns the index corresponding to the cell at position `(row, column)`
    /// on the linear array representation of the Universe, which is stored on
    /// WebAssembly's linear memory (with a byte for each cell).
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Count the number of alive neighbors of the cell at position `(row, column)`.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // @Note: we use the modulo operator `%` since neighbors
        // wrap around the Universe edges. Because of it, adding
        // `self.height - 1` or `self.width - 1` has the same effect
        // as if we were to apply a delta of `-1` (but isn't prone to
        // unsigned integer underflow).
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 { continue; }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    /// Compute one iteration of the "Game of Life".
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbors
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, n) if n < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation.
                    (Cell::Alive, n) if n > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Returns a string representation of the current state of the Universe,
    /// composed of '◻'s and '◼'s UTF-8 characters.
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Returns a new 64x64 Universe.
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let cells = (0..(width * height))
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe { width, height, cells }
    }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", if cell == Cell::Dead { '◻' } else { '◼' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
