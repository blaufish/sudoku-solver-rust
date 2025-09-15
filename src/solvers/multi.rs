use crate::sudoku;
use crate::table::Table;

pub fn solve(sudoku: &mut sudoku::Sudoku) -> bool {
    solve_inner(sudoku)
}

fn restore(sudoku: &mut sudoku::Sudoku, restorepoint: &Vec<(usize, usize)>) {
    for (row, col) in restorepoint {
        sudoku.board[*row][*col] = 0;
    }
}

fn solve_inner(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut restorepoint: Vec<(usize, usize)> = Vec::new();
    let result = solve_inner_inner(sudoku, &mut restorepoint);
    if !result {
        restore(sudoku, &mut restorepoint);
    }
    result
}

fn solve_inner_inner(sudoku: &mut sudoku::Sudoku, restorepoint: &mut Vec<(usize, usize)>) -> bool {
    let check = pre(sudoku, restorepoint);
    match check {
        PreCheckValue::Completed => return true,
        PreCheckValue::NotCompleted => (),
    }
    let moves = next_moves(sudoku);
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
    for binary in values {
        sudoku.board[row][col] = binary;
        if solve_inner(sudoku) {
            return true;
        }
        sudoku.board[row][col] = 0;
    }
    false
}

//Prioritize solving easy squares early, hopefully reducing level of recursion
fn next_moves(sudoku: &sudoku::Sudoku) -> Option<(usize, usize, Vec<u32>)> {
    let mut table = Table::new();
    table.populate(sudoku);
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

enum PreCheckValue {
    Completed,
    NotCompleted,
}

fn deduce_cell_locked_obvious(
    sudoku: &mut sudoku::Sudoku,
    table: &mut Table,
    restorepoint: &mut Vec<(usize, usize)>,
) {
    //println!("deduce_cell_locked_obvious");
    loop {
        let mut done = true;
        for grid_row in 0..(sudoku.dimensions / sudoku.grid_height) {
            for grid_col in 0..(sudoku.dimensions / sudoku.grid_width) {
                let row_base = grid_row * sudoku.grid_height;
                let col_base = grid_col * sudoku.grid_width;
                for r in 0..sudoku.grid_height {
                    for c in 0..sudoku.grid_width {
                        let row = row_base + r;
                        let col = col_base + c;
                        if sudoku.board[row][col] != 0 {
                            continue;
                        }

                        let utilized_grid = table.grids[grid_row][grid_col];
                        let utilized_row = table.rows[row];
                        let utilized_col = table.cols[col];
                        let utilized = utilized_row | utilized_col | utilized_grid;
                        let mut binary: u32 = 0;
                        let mut count = 0;
                        for i in 0..sudoku.dimensions {
                            let bin: u32 = 1 << i;
                            if bin & utilized != 0 {
                                continue;
                            }
                            binary = bin;
                            count = count + 1;
                        }

                        if count == 1 {
                            //println!("Fill in! {} {} {}", row, col, binary);
                            sudoku.board[row][col] = binary;
                            table.rows[row] ^= binary;
                            table.cols[col] ^= binary;
                            table.grids[grid_row][grid_col] ^= binary;
                            restorepoint.push((row, col));
                            done = false;
                        }
                    }
                }
            }
        }
        if done {
            break;
        }
    }
}

fn deduce_completed(sudoku: &sudoku::Sudoku) -> bool {
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] == 0 {
                return false;
            }
        }
    }
    true
}

fn pre(sudoku: &mut sudoku::Sudoku, restorepoint: &mut Vec<(usize, usize)>) -> PreCheckValue {
    let mut table = Table::new();
    table.populate(sudoku);
    if deduce_completed(&sudoku) {
        return PreCheckValue::Completed;
    }
    deduce_cell_locked_obvious(sudoku, &mut table, restorepoint);
    if deduce_completed(&sudoku) {
        return PreCheckValue::Completed;
    }
    PreCheckValue::NotCompleted
}
