use std::fs;
use std::io;
use std::time::Instant;

use clap::Parser;

mod helpers;
mod solvers;
mod sudoku;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: std::path::PathBuf,

    #[arg(long, default_value = None)]
    strategy: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let strategy = args.strategy.as_deref();

    let contents = fs::read_to_string(args.file)?;

    println!("File contents:\n{}", contents);

    let mut sudoku = helpers::parse(contents)?;

    helpers::printsudoku(&sudoku);

    let start = Instant::now();
    let solved = solvers::solve(&mut sudoku, strategy);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("Solved: {}", solved);
    helpers::printsudoku(&sudoku);
    Ok(())
}
