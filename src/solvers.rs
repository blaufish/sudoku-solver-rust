use crate::sudoku;

const MAX_GRID_DIMENSIONS: usize = 5;
const MAX_DIMENSIONS: usize = MAX_GRID_DIMENSIONS * MAX_GRID_DIMENSIONS;

pub fn solve(sudoku: &mut sudoku::Sudoku, strategy: Option<&str>) -> bool {
    let strat;
    match strategy {
        None => return solve_faster(sudoku),
        Some(s) => strat = s,
    }
    match strat {
        "backtrack_faster" => return solve_faster(sudoku),
        //"backtrack_basic" => return solve_basic(sudoku),
        _ => {
            println!("Unimplemented: {}", strat);
        }
    }
    false
}

struct Table {
    rows: [u32; MAX_DIMENSIONS],
    cols: [u32; MAX_DIMENSIONS],
    grids: [[u32; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
}

impl Table {
    fn new() -> Table {
        Table {
            rows: [0; MAX_DIMENSIONS],
            cols: [0; MAX_DIMENSIONS],
            grids: [[0; MAX_GRID_DIMENSIONS]; MAX_GRID_DIMENSIONS],
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

//Prioritize solving easy squares early, hopefully reducing level of recursion
fn next_moves(sudoku: &sudoku::Sudoku, table: &Table) -> Option<(usize, usize, Vec<u32>)> {
    let mut result: Option<(usize, usize, Vec<u32>)> = None;
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
            let mut moves: Vec<u32> = Vec::new();
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
                    } else {
                        result = Some((r, c, old_moves));
                    }
                }
            }
        }
    }
    result
}

fn solve_faster(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_faster_inner(sudoku, &mut table)
}

fn solve_faster_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table) -> bool {
    let mut solved = true;

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
        return true;
    }

    let moves = next_moves(&sudoku, &table);
    let row: usize;
    let col: usize;
    let values: Vec<u32>;
    if let Some((r, c, v)) = moves {
        row = r;
        col = c;
        values = v;
    } else {
        //Give up. No move is possible.
        return false;
    }

    let grid_row = row / sudoku.grid_height;
    let grid_col = col / sudoku.grid_width;
    for binary in values {
        sudoku.board[row][col] = binary;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;

        let recursive_solved = solve_faster_inner(sudoku, table);
        if recursive_solved {
            return true;
        }

        sudoku.board[row][col] = 0;
        table.rows[row] ^= binary;
        table.cols[col] ^= binary;
        table.grids[grid_row][grid_col] ^= binary;
    }
    false
}
