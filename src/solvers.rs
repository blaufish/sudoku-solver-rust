use crate::sudoku;
mod backtrack;

pub fn solve(sudoku: &mut sudoku::Sudoku, strategy: Option<&str>) -> bool {
    let strat;
    match strategy {
        None => return backtrack::solve(sudoku),
        Some(s) => strat = s,
    }
    match strat {
        "backtrack_faster" => return backtrack::solve(sudoku),
        _ => {
            println!("Unimplemented: {}", strat);
        }
    }
    false
}
