use crate::sudoku;
use crate::table::Table;

pub fn solve(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut table = Table::new();
    table.populate(sudoku);
    solve_inner(sudoku, &mut table)
}

//Prioritize solving easy squares early, hopefully reducing level of recursion
fn next_moves(sudoku: &sudoku::Sudoku, table: &Table) -> Option<(usize, usize, Vec<u64>)> {
    let mut result: Option<(usize, usize, Vec<u64>)> = None;
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            if sudoku.board[row][col] != 0 {
                continue;
            }
            let utilized = table.get_utilized_rc(row, col);
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

fn solve_inner(sudoku: &mut sudoku::Sudoku, table: &mut Table) -> bool {
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
    let values: Vec<u64>;
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
        table.toggle_grgc_rc(grid_row, grid_col, row, col, binary);

        let recursive_solved = solve_inner(sudoku, table);
        if recursive_solved {
            return true;
        }

        sudoku.board[row][col] = 0;
        table.toggle_grgc_rc(grid_row, grid_col, row, col, binary);
    }
    false
}
