use crate::sudoku;
//use crate::sudoku::MAX_DIMENSIONS;
//use crate::sudoku::MAX_GRID_DIMENSIONS;
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
    let check = pre(sudoku, &mut restorepoint);
    match check {
        PreCheckValue::Failed => {
            //println!("failed...");
            restore(sudoku, &restorepoint);
            return false;
        }
        PreCheckValue::Completed => return true,
        PreCheckValue::NotCompleted => (),
    }
    let moves = next_moves(&sudoku);
    let row: usize;
    let col: usize;
    let values: Vec<u32>;
    if let Some((r, c, v)) = moves {
        row = r;
        col = c;
        values = v;
    } else {
        //Give up. No move is possible.
        restore(sudoku, &restorepoint);
        return false;
    }
    for binary in values {
        sudoku.board[row][col] = binary;
        if solve_inner(sudoku) {
            return true;
        }
        sudoku.board[row][col] = 0;
    }

    //println!("default return, restoring...");
    restore(sudoku, &restorepoint);
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

fn bincnt(bin: u32) -> u32 {
    let mut cnt = 0;
    let mut b = bin;
    for i in 0..31 {
        if b & (1 << i) != 0 {
            cnt = cnt + 1;
        }
    }
    cnt
}

enum PreCheckValue {
    Completed,
    Failed,
    NotCompleted,
}

fn calculate_utilized(sudoku: &sudoku::Sudoku, table: &Table, row: usize, col: usize) -> u32 {
    let grid_row = row / sudoku.grid_height;
    let grid_col = col / sudoku.grid_width;
    let utilized_grid = table.grids[grid_row][grid_col];
    let utilized_row = table.rows[row];
    let utilized_col = table.cols[col];
    let utilized = utilized_row | utilized_col | utilized_grid;
    utilized
}

fn calculate_not_utilized(sudoku: &sudoku::Sudoku, table: &Table, row: usize, col: usize) -> u32 {
    let utilized = calculate_utilized(sudoku, table, row, col);
    let xor_pattern: u32 = (1 << sudoku.character_set.chars().count()) - 1;
    let inverted = utilized ^ xor_pattern;
    return inverted;
}

fn deduce_broken(sudoku: &mut sudoku::Sudoku, table: &Table) -> bool {
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] != 0 {
                continue;
            }
            let not_utilized = calculate_not_utilized(&sudoku, &table, row, col);
            if not_utilized == 0 {
                return true;
            }
        }
    }
    return false;
}

fn deduce_cell_locked_obvious(
    sudoku: &mut sudoku::Sudoku,
    table: &mut Table,
    restorepoint: &mut Vec<(usize, usize)>,
) {
    //println!("deduce_cell_locked_obvious");
    loop {
        let mut done = true;
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
    //if deduce_broken(sudoku, &table) {
    //    return PreCheckValue::Failed;
    //}
    deduce_cell_locked_obvious(sudoku, &mut table, restorepoint);
    if deduce_completed(&sudoku) {
        return PreCheckValue::Completed;
    }
    PreCheckValue::NotCompleted
}
