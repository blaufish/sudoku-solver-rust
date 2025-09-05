use crate::sudoku;
use crate::sudoku::MAX_DIMENSIONS;
use crate::sudoku::MAX_GRID_DIMENSIONS;

pub struct Table {
    pub rows: [u32; MAX_DIMENSIONS],
    pub cols: [u32; MAX_DIMENSIONS],
    pub grids: [[u32; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: [0; MAX_DIMENSIONS],
            cols: [0; MAX_DIMENSIONS],
            grids: [[0; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
        }
    }
    pub fn populate(&mut self, sudoku: &sudoku::Sudoku) {
        for i in 0..sudoku.dimensions {
            self.rows[i] = sudoku.utilized_row(i);
            self.cols[i] = sudoku.utilized_col(i);
        }
        for r in 0..(sudoku.dimensions / sudoku.grid_height) {
            for c in 0..(sudoku.dimensions / sudoku.grid_width) {
                let row = r * sudoku.grid_height;
                let col = c * sudoku.grid_width;
                self.grids[r][c] = sudoku.utilized_grid(row, col);
            }
        }
    }
}
