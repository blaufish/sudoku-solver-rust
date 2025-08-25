use crate::sudoku;

pub fn solve(sudoku: &mut sudoku::Sudoku, strategy: Option<&str>) -> bool {
    let strat;
    match strategy {
        None => return solve_faster(sudoku),
        Some(s) => strat = s,
    }
    match strat {
        "backtrack_faster" => return solve_faster(sudoku),
        "backtrack_basic" => return solve_basic(sudoku),
        _ => {
            println!("Unimplemented: {}", strat);
        }
    }
    false
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
        for r in 0..(sudoku.dimensions / sudoku.subsquare_height) {
            for c in 0..(sudoku.dimensions / sudoku.subsquare_width) {
                let row = r * sudoku.subsquare_height;
                let col = c * sudoku.subsquare_width;
                self.grids[r][c] = sudoku.utilized_subsuqare(row, col);
            }
        }
    }
}

fn solve_faster(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_faster_inner(sudoku, &mut table)
}

fn solve_faster_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table) -> bool {
    let mut solved = true;
    let mut row: usize = 0;
    let mut col: usize = 0;

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
        return true;
    }
    let utilized_row = table.rows[row];
    let utilized_col = table.cols[col];
    let grid_row = row / sudoku.subsquare_height;
    let grid_col = col / sudoku.subsquare_width;
    let utilized_subsuqare = table.grids[grid_row][grid_col];
    let utilized = utilized_row | utilized_col | utilized_subsuqare;
    for i in 0..sudoku.dimensions {
        let binary: u32 = 1 << i;
        if binary & utilized != 0 {
            continue;
        }

        sudoku.board[row][col] = binary;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        let recursive_solved = solve_faster_inner(sudoku, table);
        if recursive_solved {
            return true;
        } else {
            sudoku.board[row][col] = 0;
            table.rows[row] ^= binary;
            table.cols[col] ^= binary;
            table.grids[grid_row][grid_col] ^= binary;
        }
    }
    false
}

fn solve_basic(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut solved = true;
    let mut row: usize = 0;
    let mut col: usize = 0;

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
        return true;
    }
    let utilized_row = sudoku.utilized_row(row);
    let utilized_col = sudoku.utilized_col(col);
    let utilized_subsuqare = sudoku.utilized_subsuqare(row, col);
    let utilized = utilized_row | utilized_col | utilized_subsuqare;
    for i in 0..sudoku.dimensions {
        let binary: u32 = 1 << i;
        if binary & utilized != 0 {
            continue;
        }
        sudoku.board[row][col] = binary;
        let recursive_solved = solve_basic(sudoku);
        if recursive_solved {
            return true;
        } else {
            sudoku.board[row][col] = 0;
        }
    }
    false
}
