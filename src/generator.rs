use crate::sudoku;
use rand::prelude::*;
//use std::io::Write;
use std::time::Duration;
use std::time::Instant;

pub struct Generator {
    pub dimensions: usize,
    pub grid_width: usize,
    pub grid_height: usize,
    pub charset: String,
    pub max_prune_seconds : u64,
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

fn get_random_grid(dimensions: usize) -> Vec<u32>  {
    let mut rng = rand::rng();
    let mut vec : Vec<u32> = Vec::new();
    let mut binary : u32 = 1;
    for _i in 0..dimensions {
        vec.push(binary);
        binary = binary << 1;
    }
    _ = vec.shuffle(&mut rng);
    return vec;
}

fn fill_grid(sudoku: &mut sudoku::Sudoku, grid_row: usize, grid_col: usize) {
    let random_grid : Vec<u32> = get_random_grid(sudoku.dimensions);
    for i in 0..sudoku.dimensions {
        let val : u32 = random_grid[i];
        let row = grid_row * sudoku.grid_height + (i / sudoku.grid_width);
        let col = grid_col * sudoku.grid_width + (i % sudoku.grid_width);
        //println!("Init: {},{} = {}", row, col, val);
        sudoku.board[row][col] = val;
    }
}

pub fn generate_golden(generator: &Generator) -> Option<sudoku::Sudoku> {
    let mut sudoku = sudoku::Sudoku::new(
        generator.dimensions,
        generator.grid_height,
        generator.grid_width,
        generator.charset.clone(),
    );
    let grid_dim;
    if generator.grid_width > generator.grid_height {
        grid_dim = generator.grid_height;
    }
    else {
        grid_dim = generator.grid_width;
    }

    let mut cells = get_empty_cells(sudoku.clone());
    let mut rng = rand::rng();
    cells.shuffle(&mut rng);

    for i in 0..grid_dim {
        fill_grid(&mut sudoku, i, i);
    }
    let mut ignore : Vec<sudoku::Sudoku> = Vec::new();
    let vec = solve(&mut sudoku, 1, &mut ignore);
    for v in vec {
        return Some(v);
    }
    //println!("Could not find any solution?");
    None
}

pub fn get_empty_cells(sudoku: sudoku::Sudoku) -> Vec<(usize, usize)> {
    let mut v : Vec<(usize, usize)> = Vec::new();
    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            let row : usize = r;
            let col : usize = c;
            if sudoku.board[row][col] == 0 {
                v.push((row, col));
            }
        }
    }
    v
}

pub fn get_none_empty_cells(sudoku: sudoku::Sudoku) -> Vec<(usize, usize)> {
    let mut v : Vec<(usize, usize)> = Vec::new();
    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            let row : usize = r;
            let col : usize = c;
            if sudoku.board[row][col] != 0 {
                v.push((row, col));
            }
        }
    }
    v
}

pub fn generate_challenge(generator: &Generator, golden: &sudoku::Sudoku) -> Option<sudoku::Sudoku> {
    let max_duration = Duration::new(generator.max_prune_seconds, 0);

    let mut sudoku = golden.clone();
    let mut rng = rand::rng();

    let mut ignore : Vec<sudoku::Sudoku> = Vec::new();
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
        let row : usize;
        let col : usize;
        //row = cell.row;
        //col = cell.col;
        (row, col) = cell;
        let tmp = sudoku.board[row][col];
        sudoku.board[row][col] = 0;
        let vec = solve(&mut sudoku, 1, &mut ignore);
        if vec.len() != 0 {
            sudoku.board[row][col] = tmp;
        }
    }
    return Some(sudoku);
}

struct Table {
    rows: [u32; 25],
    cols: [u32; 25],
    grids: [[u32; 5]; 5],
}

impl Table {
    fn new() -> Table {
        Table {
            rows: [0; 25],
            cols: [0; 25],
            grids: [[0; 5]; 5],
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

fn sudoku_equals(a: &sudoku::Sudoku, b: &sudoku::Sudoku) -> bool {
    if a.dimensions != b.dimensions {
        return false;
    }
    if a.grid_height != b.grid_height {
        return false;
    }
    if a.grid_width != b.grid_width {
        return false;
    }
    for row in 0..a.dimensions {
        for col in 0..a.dimensions {
            if a.board[row][col] != b.board[row][col] {
                return false;
            }
        }
    }
    true
}

fn solve(sudoku: &mut sudoku::Sudoku, max_entries: usize, ignore: &mut Vec<sudoku::Sudoku>) -> Vec<sudoku::Sudoku> {
    let mut table = Table::new();
    table.populate(sudoku);

    let result = solve_inner(sudoku, &mut table, max_entries, ignore);
    //let mut to_be_ignored = result.clone();
    //ignore.append(&mut to_be_ignored);
    result
}

//Prioritize solving easy squares early, hopefully reducing level of recursion
fn next_moves(sudoku: &sudoku::Sudoku, table: &Table) -> Option<(usize, usize, Vec<u32>)> {
    let mut result : Option<(usize, usize, Vec<u32>)> = None;
    for row in 0..sudoku.dimensions {
        let utilized_row = table.rows[row];
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] != 0 {
                continue;
            }
            let utilized_col = table.cols[col];
            let grid_row = row / sudoku.grid_height;
            let grid_col = col / sudoku.grid_width;
            let utilized_grid = table.grids[grid_row][grid_col];
            let utilized = utilized_row | utilized_col | utilized_grid;
            let mut moves : Vec<u32> = Vec::new();
            for i in 0..sudoku.dimensions {
                let binary: u32 = 1 << i;
                if binary & utilized != 0 {
                    continue;
                }
                moves.push(binary);
            }
            if moves.len() == 0 {
                //Board is in a bad state, a cell cannot accept any moves
                return None;
            }
            match result {
                None => result = Some((row, col, moves)),
                Some((r, c, old_moves)) => {
                    if moves.len() < old_moves.len() {
                        result = Some((row, col, moves));
                    }
                    else {
                        result = Some((r, c, old_moves));
                    }
                }
            }
        }
    }
    result
}

fn solve_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table, max_entries: usize, ignore: &Vec<sudoku::Sudoku>) -> Vec<sudoku::Sudoku> {
    let mut vec: Vec<sudoku::Sudoku> = Vec::new();
    let mut solved = true;

    for bad in ignore {
        if sudoku_equals(&bad, &sudoku) {
            //println!("hrrm");
            return vec;
        }
    }

    for r in 0..sudoku.dimensions {
        for c in 0..sudoku.dimensions {
            if sudoku.board[r][c] != 0 {
                continue;
            }
            solved = false;
            break;
        }
    }
    if solved {
        //println!("Solved!");
        vec.push(sudoku.clone());
        return vec;
    }


    let moves = next_moves(&sudoku, &table);
    let row : usize;
    let col : usize;
    let values : Vec<u32>;
    if let Some((r, c, v)) = moves {
        row = r;
        col = c;
        values = v;
    }
    else {
        //Give up. No move is possible.
        return vec;
    }

    let grid_row = row / sudoku.grid_height;
    let grid_col = col / sudoku.grid_width;
    for binary in values {
        sudoku.board[row][col] = binary;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        let recursive_solved = solve_inner(sudoku, table, max_entries, ignore);

        sudoku.board[row][col] = 0;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        //if recursive_solved.len() > 0 {
        //    println!("Solutions... {}", recursive_solved.len());
        //}
        for v in recursive_solved {
            vec.push(v);
            //println!("vec.len(): {}", vec.len());
            if vec.len() >= max_entries {
                return vec;
            }
        }
    }
    vec
}
