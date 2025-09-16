use crate::solvers::solve;
use crate::sudoku;

use rand::prelude::*;
use std::time::Duration;
use std::time::Instant;

pub struct Generator {
    pub dimensions: usize,
    pub grid_width: usize,
    pub grid_height: usize,
    pub charset: String,
    pub max_prune_seconds: u64,
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

fn get_random_grid(dimensions: usize) -> Vec<u64> {
    let mut rng = rand::rng();
    let mut vec: Vec<u64> = Vec::new();
    let mut binary: u64 = 1;
    for _i in 0..dimensions {
        vec.push(binary);
        binary = binary << 1;
    }
    _ = vec.shuffle(&mut rng);
    return vec;
}

fn fill_grid(sudoku: &mut sudoku::Sudoku, grid_row: usize, grid_col: usize) {
    let random_grid: Vec<u64> = get_random_grid(sudoku.dimensions);
    for i in 0..sudoku.dimensions {
        let val: u64 = random_grid[i];
        let row = grid_row * sudoku.grid_height + (i / sudoku.grid_width);
        let col = grid_col * sudoku.grid_width + (i % sudoku.grid_width);
        //println!("Init: {},{} = {}", row, col, val);
        sudoku.board[row][col] = val;
    }
}

pub fn generate_golden(generator: &Generator) -> Option<sudoku::Sudoku> {
    if generator.dimensions > 4 {
        return generate_golden_large(generator);
    } else {
        return generate_golden_small(generator);
    }
}

fn generate_golden_small(generator: &Generator) -> Option<sudoku::Sudoku> {
    let mut sudoku = sudoku::Sudoku::new(
        generator.dimensions,
        generator.grid_height,
        generator.grid_width,
        generator.charset.clone(),
    );
    for r in 0..generator.dimensions / generator.grid_height {
        for c in 0..generator.dimensions / generator.grid_width {
            let sudoku_reset = sudoku.clone();
            fill_grid(&mut sudoku, r, c);
            let sudoku_new = sudoku.clone();
            let solved = solve(&mut sudoku, None);
            if solved {
                let (valid, _) = sudoku.validate();
                //println!("valid: {}", valid);
                if valid {
                    sudoku = sudoku_new.clone();
                } else {
                    sudoku = sudoku_reset.clone();
                }
            } else {
                sudoku = sudoku_reset.clone();
            }
        }
    }
    {
        let solved = solve(&mut sudoku, None);
        match solved {
            false => None,
            true => Some(sudoku),
        }
    }
}

fn generate_golden_large(generator: &Generator) -> Option<sudoku::Sudoku> {
    let mut sudoku = sudoku::Sudoku::new(
        generator.dimensions,
        generator.grid_height,
        generator.grid_width,
        generator.charset.clone(),
    );
    let grid_dim;
    if generator.grid_width > generator.grid_height {
        grid_dim = generator.grid_height;
    } else {
        grid_dim = generator.grid_width;
    }

    let mut cells = get_empty_cells(sudoku.clone());
    let mut rng = rand::rng();
    cells.shuffle(&mut rng);

    for i in 0..grid_dim {
        fill_grid(&mut sudoku, i, i);
    }
    let solved = solve(&mut sudoku, None);
    if !solved {
        return None;
    }
    Some(sudoku)
}

fn get_empty_cells(sudoku: sudoku::Sudoku) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            let row: usize = r;
            let col: usize = c;
            if sudoku.board[row][col] == 0 {
                v.push((row, col));
            }
        }
    }
    v
}

fn get_none_empty_cells(sudoku: sudoku::Sudoku) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            let row: usize = r;
            let col: usize = c;
            if sudoku.board[row][col] != 0 {
                v.push((row, col));
            }
        }
    }
    v
}

fn try_remove(sudoku: &mut sudoku::Sudoku, row: usize, col: usize) {
    let tmp = sudoku.board[row][col];
    let charset_len = sudoku.character_set.chars().count();
    let mut sudoku2 = sudoku.clone();

    sudoku2.board[row][col] = 0;
    let utilized_grid = sudoku2.utilized_grid(row, col);
    let utilized_row = sudoku2.utilized_row(row);
    let utilized_col = sudoku2.utilized_col(col);
    let utilized = utilized_grid | utilized_row | utilized_col;

    for i in 0..charset_len {
        let binary: u64 = 1 << i;
        if tmp == binary {
            // This is the correct sudoku, no need to validate.
            continue;
        }
        if utilized & binary != 0 {
            //This value cannot be picked, not an option!
            continue;
        }
        sudoku2.board[row][col] = binary;
        let solved = solve(&mut sudoku2, None);
        if solved {
            //An alernative solution was found, this branch is poisoned!
            return;
        }
    }
    //No wrong solutions found, unfill this cell.
    sudoku.board[row][col] = 0;
}

pub fn generate_challenge(
    generator: &Generator,
    golden: &sudoku::Sudoku,
) -> Option<sudoku::Sudoku> {
    let max_duration = Duration::new(generator.max_prune_seconds, 0);

    let mut sudoku = golden.clone();
    let mut rng = rand::rng();

    let mut ignore: Vec<sudoku::Sudoku> = Vec::new();
    ignore.push(golden.clone());
    let start = Instant::now();

    let mut cells = get_none_empty_cells(sudoku.clone());
    cells.shuffle(&mut rng);
    for cell in cells {
        let duration = start.elapsed();
        //println!("Elapsed {}...", duration.as_secs());
        if duration > max_duration {
            break;
        }
        let row: usize;
        let col: usize;
        //row = cell.row;
        //col = cell.col;
        (row, col) = cell;

        try_remove(&mut sudoku, row, col);
    }
    return Some(sudoku);
}
