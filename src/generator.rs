use crate::sudoku;
use rand::prelude::*;

pub struct Generator {
    pub dimensions: usize,
    pub grid_width: usize,
    pub grid_height: usize,
    pub charset: String,
    pub threshold: usize,
    pub picks_per_solve: usize,
    pub kickstart_cells: usize,
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

fn _possible_binaries(sudoku: &sudoku::Sudoku, row: usize, col: usize) -> Vec<u32> {
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

//find where alternatives deviates from the golden path.
//suggest improvements to sudoku to approach the golden path
fn get_diff(sudoku: &sudoku::Sudoku, golden: &sudoku::Sudoku, alternative : &sudoku::Sudoku) -> Vec<(usize, usize, u32)> {
    let mut diff: Vec<(usize, usize, u32)> = Vec::new();
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] != 0 {
                continue;
            }
            if golden.board[row][col] == alternative.board[row][col] {
                continue;
            }
            diff.push((row, col, golden.board[row][col]));
        }
    }
    diff
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

fn generate_golden(generator: &Generator) -> Option<sudoku::Sudoku> {
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
    for _i in 0..grid_dim {
        let vec : Vec<u32> = get_random_grid(generator.dimensions);
        let row = grid_dim * generator.grid_height;
        let cel = grid_dim * generator.grid_width;
        for rc in 0..generator.dimensions {
            let rc_row = row + rc / generator.grid_width;
            let rc_cel = cel + rc % generator.grid_width;
            sudoku.board[rc_row][rc_cel] = vec[rc];
        }
    }
    let vec = solve(&mut sudoku, 1);
    for v in vec {
        return Some(v);
    }
    None
}

pub fn generate(generator: &Generator) -> Option<(sudoku::Sudoku, sudoku::Sudoku)> {
    let mut sudoku = sudoku::Sudoku::new(
        generator.dimensions,
        generator.grid_height,
        generator.grid_width,
        generator.charset.clone(),
    );
    let golden;
    if let Some(g) = generate_golden(generator) {
        golden = g;
    }
    else {
        return None;
    }

    for _i in 0..generator.kickstart_cells {
        let row = rand::random_range(0..generator.dimensions);
        let col = rand::random_range(0..generator.dimensions);
        if sudoku.board[row][col] != 0 {
            continue;
        }
        sudoku.board[row][col] = golden.board[row][col];
    }
    let mut rng = rand::rng();
    loop {
        let vec = solve(&mut sudoku, generator.threshold);
        if vec.len() == 0 {
            return None;
        }
        if vec.len() == 1 {
            for sudoku2 in vec {
                return Some((sudoku, sudoku2));
            }
            return None;
        }
        match vec.choose(&mut rng) {
            None => return None,
            Some(alternative) => {
                for _i in 0..generator.picks_per_solve {
                    let diffs = get_diff(&sudoku, &golden, &alternative);
                    if let Some((dr, dc, dv)) = diffs.choose(&mut rng) {
                        sudoku.board[*dr][*dc] = *dv;
                    }
                }
            }
        }
    }
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

fn solve(sudoku: &mut sudoku::Sudoku, max_entries: usize) -> Vec<sudoku::Sudoku> {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_inner(sudoku, &mut table, max_entries)
}

fn solve_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table, max_entries: usize) -> Vec<sudoku::Sudoku> {
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

        let recursive_solved = solve_inner(sudoku, table, max_entries);

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
