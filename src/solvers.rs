use crate::sudoku;
mod backtrack;
mod multi;

const BACKTRACK: &str = "backtrack";
const MULTI: &str = "mutli";

const DEFAULT: &str = MULTI;

pub fn list_solvers() -> Vec<String> {
    return vec![MULTI.to_string(), BACKTRACK.to_string()];
}

pub fn solve(sudoku: &mut sudoku::Sudoku, strategy: Option<&str>) -> bool {
    let strat;
    match strategy {
        None => strat = DEFAULT,
        Some(s) => strat = s,
    }
    match strat {
        BACKTRACK => return backtrack::solve(sudoku),
        MULTI => return multi::solve(sudoku),
        _ => {
            println!("Unimplemented: {}", strat);
        }
    }
    false
}
