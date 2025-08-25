use crate::sudoku;
use rand::prelude::*;

pub struct Generator {
    pub dimensions: usize,
    pub grid_width: usize,
    pub grid_height: usize,
    pub charset: String,
}

impl Generator {
    pub fn validate_generator(&self) -> bool {
        if self.dimensions != self.grid_height * self.grid_width {
            println!(
                "Error: size mismatch, {} != {}*{}",
                self.dimensions, self.grid_height, self.grid_width
            );
            return false;
        }
        if self.dimensions % self.grid_height != 0 {
            println!(
                "Error: size mismatch, {} cannot be divided by {}",
                self.dimensions, self.grid_height
            );
        }
        if self.dimensions % self.grid_width != 0 {
            println!(
                "Error: size mismatch, {} cannot be divided by {}",
                self.dimensions, self.grid_width
            );
        }
        let charset_len = self.charset.chars().count();
        if self.dimensions != charset_len {
            println!(
                "Error: charset mismatch, required length {}, got {} {}",
                self.dimensions, charset_len, self.charset
            );
            return false;
        }
        true
    }
}

fn possible_binaries(sudoku: &sudoku::Sudoku, row: usize, col: usize) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    let mut binary: u32 = 1;
    let utilized =
        sudoku.utilized_row(row) | sudoku.utilized_col(col) | sudoku.utilized_grid(row, col);
    for _i in 0..sudoku.dimensions {
        if binary & utilized == 0 {
            vec.push(binary);
            binary = binary << 1;
        }
    }
    vec
}

fn get_diff(sudoku: &sudoku::Sudoku, vec: Vec<sudoku::Sudoku>) -> Vec<(usize, usize, u32)> {
    let mut diff: Vec<(usize, usize, u32)> = Vec::new();
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] != 0 {
                continue;
            }
            for v in vec.clone() {
                diff.push((row, col, v.board[row][col]));
            }
        }
    }
    diff
}

pub fn generate(generator: &Generator) {
    let mut sudoku = sudoku::Sudoku::new(
        generator.dimensions,
        generator.grid_height,
        generator.grid_width,
        generator.charset.clone(),
    );
    let mut rng = rand::rng();
    for _i in 0..generator.dimensions {
        let row = rand::random_range(0..generator.dimensions);
        let col = rand::random_range(0..generator.dimensions);
        if sudoku.board[row][col] != 0 {
            continue;
        }
        let choices = possible_binaries(&sudoku, row, col);
        let binary = choices.choose(&mut rng);
        if let Some(bin) = binary {
            sudoku.board[row][col] = *bin;
        }
    }
    println!("randomized a few starting values...");
    println!("{}", &sudoku.to_string());
    loop {
        let mut vec = solve(&mut sudoku);
        //println!("got {} solutions!", vec.len());
        if vec.len() == 1 {
            for sudoku2 in vec {
                println!("Challenge:");
                println!("{}", sudoku.to_string());
                println!("Solution:");
                println!("{}", sudoku2.to_string());
            }
            break;
        }
        let diffs = get_diff(&sudoku, vec.clone());
        if let Some((dr, dc, dv)) = diffs.choose(&mut rng) {
            sudoku.board[*dr][*dc] = *dv;
            //println!("board[{}][{}] = {}", *dr, *dc, *dv);
            //println!("{}", sudoku.to_string());
            /*
            let mut pruned : Vec<sudoku::Sudoku> = Vec::new();
            for v in vec {
                if v.board[*dr][*dc] == *dv {
                    pruned.push(v);
                }
            }
            vec = pruned;
            */
        }
    }
}

struct Table {
    rows: [u32; 16],
    cols: [u32; 16],
    grids: [[u32; 4]; 4],
}

impl Table {
    fn new() -> Table {
        Table {
            rows: [0; 16],
            cols: [0; 16],
            grids: [[0; 4]; 4],
        }
    }
    fn populate(&mut self, sudoku: &sudoku::Sudoku) {
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

fn solve(sudoku: &mut sudoku::Sudoku) -> Vec<sudoku::Sudoku> {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_inner(sudoku, &mut table)
}

fn solve_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table) -> Vec<sudoku::Sudoku> {
    let mut vec: Vec<sudoku::Sudoku> = Vec::new();
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut solved = true;

    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            if sudoku.board[r][c] != 0 {
                continue;
            }
            solved = false;
            row = r;
            col = c;
            break;
        }
    }
    if solved {
        vec.push(sudoku.clone());
        return vec;
    }
    let max_entries = 10;

    let utilized_row = table.rows[row];
    let utilized_col = table.cols[col];
    let grid_row = row / sudoku.grid_height;
    let grid_col = col / sudoku.grid_width;
    let utilized_grid = table.grids[grid_row][grid_col];
    let utilized = utilized_row | utilized_col | utilized_grid;
    for i in 0..sudoku.dimensions {
        let binary: u32 = 1 << i;
        if binary & utilized != 0 {
            continue;
        }

        sudoku.board[row][col] = binary;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        let recursive_solved = solve_inner(sudoku, table);

        sudoku.board[row][col] = 0;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        for v in recursive_solved {
            vec.push(v);
            if vec.len() >= max_entries {
                return vec;
            }
        }
    }
    vec
}
