use crate::sudoku;

pub fn solve(sudoku: &mut sudoku::Sudoku, strategy: Option<&str>) -> bool {
    let strat;
    match strategy {
        None => return solve_basic(sudoku),
        Some(s) => strat = s,
    }
    match strat {
        "backtrack" => return solve_basic(sudoku),
        _ => {
            println!("Unimplemented: {}", strat);
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
