use crate::sudoku;
use crate::table::Table;

pub fn solve(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_inner(sudoku, &mut table)
}

fn restore(sudoku: &mut sudoku::Sudoku, table: &mut Table, restorepoint: &Vec<(usize, usize)>) {
    for (row, col) in restorepoint {
        let binary = sudoku.board[*row][*col];

        sudoku.board[*row][*col] = 0;

        table.toggle_rc(*row, *col, binary);
    }
}

fn solve_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table) -> bool {
    let mut restorepoint: Vec<(usize, usize)> = Vec::new();
    let result = solve_inner_inner(sudoku, table, &mut restorepoint);
    if !result {
        restore(sudoku, table, &mut restorepoint);
    }
    result
}

fn solve_inner_inner(
    sudoku: &mut sudoku::Sudoku,
    table: &mut Table,
    restorepoint: &mut Vec<(usize, usize)>,
) -> bool {
    let check = pre(sudoku, table, restorepoint);
    match check {
        PreCheckValue::Completed => return true,
        PreCheckValue::NotCompleted => (),
    }
    let moves = next_moves(sudoku, table);
    let row: usize;
    let col: usize;
    let values: Vec<u64>;
    if let Some((r, c, v)) = moves {
        row = r;
        col = c;
        values = v;
    } else {
        //Give up. No move is possible.
        return false;
    }
    for binary in values {
        let grid_row = row / sudoku.grid_height;
        let grid_col = col / sudoku.grid_width;

        sudoku.board[row][col] = binary;

        table.toggle_grgc_rc(grid_row, grid_col, row, col, binary);

        if solve_inner(sudoku, table) {
            return true;
        }

        sudoku.board[row][col] = 0;

        table.toggle_grgc_rc(grid_row, grid_col, row, col, binary);
    }
    false
}

//Prioritize solving easy squares early, hopefully reducing level of recursion
fn next_moves(sudoku: &sudoku::Sudoku, table: &Table) -> Option<(usize, usize, Vec<u64>)> {
    let mut result: Option<(usize, usize, Vec<u64>)> = None;
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

                    let utilized = table.get_utilized_grgc_rc(grid_row, grid_col, row, col);
                    let mut moves: Vec<u64> = Vec::new();
                    for i in 0..sudoku.dimensions {
                        let binary: u64 = 1 << i;
                        if binary & utilized != 0 {
                            continue;
                        }
                        moves.push(binary);
                    }
                    if moves.len() == 0 {
                        //Board is in a bad state, a cell cannot accept any moves
                        return None;
                    }
                    if moves.len() == 1 {
                        //We found easiest move, exit early
                        return Some((row, col, moves));
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

                        let utilized = table.get_utilized_grgc_rc(grid_row, grid_col, row, col);
                        let mut binary: u64 = 0;
                        let mut count = 0;
                        for i in 0..sudoku.dimensions {
                            let bin: u64 = 1 << i;
                            if bin & utilized != 0 {
                                continue;
                            }
                            binary = bin;
                            count = count + 1;
                        }

                        if count == 1 {
                            //println!("Fill in! {} {} {}", row, col, binary);
                            sudoku.board[row][col] = binary;
                            table.toggle_grgc_rc(grid_row, grid_col, row, col, binary);
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

fn pre(
    sudoku: &mut sudoku::Sudoku,
    table: &mut Table,
    restorepoint: &mut Vec<(usize, usize)>,
) -> PreCheckValue {
    if deduce_completed(&sudoku) {
        return PreCheckValue::Completed;
    }
    deduce_cell_locked_obvious(sudoku, table, restorepoint);
    if deduce_completed(&sudoku) {
        return PreCheckValue::Completed;
    }
    PreCheckValue::NotCompleted
}
