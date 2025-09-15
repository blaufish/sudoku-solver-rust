use crate::sudoku;
use crate::table::Table;

pub fn solve(
    sudoku: &mut sudoku::Sudoku,
    max_entries: usize,
    ignore: &mut Vec<sudoku::Sudoku>,
) -> Vec<sudoku::Sudoku> {
    let mut table = Table::new();
    table.populate(sudoku);

    let result = solve_inner(sudoku, &mut table, max_entries, ignore);
    //let mut to_be_ignored = result.clone();
    //ignore.append(&mut to_be_ignored);
    result
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

fn solve_inner(
    sudoku: &mut sudoku::Sudoku,
    table: &mut Table,
    max_entries: usize,
    ignore: &Vec<sudoku::Sudoku>,
) -> Vec<sudoku::Sudoku> {
    let mut vec: Vec<sudoku::Sudoku> = Vec::new();
    let mut solved = true;

    for bad in ignore {
        if sudoku.equals(&bad) {
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
    let row: usize;
    let col: usize;
    let values: Vec<u32>;
    if let Some((r, c, v)) = moves {
        row = r;
        col = c;
        values = v;
    } else {
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
