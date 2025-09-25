use crate::sudoku;
use crate::sudoku::MAX_DIMENSIONS;
use crate::sudoku::MAX_GRID_DIMENSIONS;

pub struct Table {
    pub rows: [u64; MAX_DIMENSIONS],
    pub cols: [u64; MAX_DIMENSIONS],
    pub grids: [[u64; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
    pub grid_height: usize,
    pub grid_width: usize,
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: [0; MAX_DIMENSIONS],
            cols: [0; MAX_DIMENSIONS],
            grids: [[0; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
            grid_height: 0,
            grid_width: 0,
        }
    }
    pub fn populate(&mut self, sudoku: &sudoku::Sudoku) {
        self.grid_height = sudoku.grid_height;
        self.grid_width = sudoku.grid_width;
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

    pub fn toggle_grgc_rc(
        &mut self,
        grid_row: usize,
        grid_col: usize,
        row: usize,
        col: usize,
        binary: u64,
    ) {
        self.rows[row] ^= binary;
        self.cols[col] ^= binary;
        self.grids[grid_row][grid_col] ^= binary;
    }

    pub fn toggle_rc(&mut self, row: usize, col: usize, binary: u64) {
        let grid_row = row / self.grid_height;
        let grid_col = col / self.grid_width;
        self.toggle_grgc_rc(grid_row, grid_col, row, col, binary);
    }

    pub fn get_utilized_grgc_rc(
        &self,
        grid_row: usize,
        grid_col: usize,
        row: usize,
        col: usize,
    ) -> u64 {
        let utilized_row = self.rows[row];
        let utilized_col = self.cols[col];
        let utilized_grid = self.grids[grid_row][grid_col];
        let utilized = utilized_row | utilized_col | utilized_grid;
        utilized
    }

    pub fn get_utilized_rc(&self, row: usize, col: usize) -> u64 {
        let grid_row = row / self.grid_height;
        let grid_col = col / self.grid_width;
        let utilized = self.get_utilized_grgc_rc(grid_row, grid_col, row, col);
        utilized
    }
}
