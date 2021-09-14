mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen]
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let mut cells: Vec<Cell> = vec![];
        for i in 0..width * height {
            if i%2 == 0 || i%7 == 0 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }
        Universe{
            width,
            height,
            cells
        }
    }
    #[wasm_bindgen]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[wasm_bindgen]
    pub fn height(&self) -> u32 {
        self.height
    }
    #[wasm_bindgen]
    pub fn cells(&self) ->  *const Cell {
        self.cells.as_ptr()
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn get_live_neigbhor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                let neigbhor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let idx =  self.get_index(neigbhor_row, neighbor_column);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.get_live_neigbhor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    #[wasm_bindgen]
    pub fn render(&self)-> String {
        self.to_string()
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
      Ok(())
    }
}
