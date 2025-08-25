use std::fs;
use std::io;

mod helpers;
mod solvers;
mod sudoku;

fn main() -> io::Result<()> {
    let file_path = "challenge.txt";

    let contents = fs::read_to_string(file_path)?;

    println!("File contents:\n{}", contents);

    let mut sudoku = helpers::parse(contents)?;

    helpers::printsudoku(&sudoku);
    let solved = solvers::solve(&mut sudoku, None);
    println!("Solved: {}", solved);
    helpers::printsudoku(&sudoku);
    Ok(())
}
