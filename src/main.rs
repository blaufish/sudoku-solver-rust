use std::fs;
use std::io;
use std::time::Instant;

use clap::Parser;

mod generator;
mod helpers;
mod solvers;
mod sudoku;
mod table;
mod tests;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    file: Option<std::path::PathBuf>,

    #[arg(long, default_value = "false")]
    solve: bool,

    #[arg(long, default_value = "false")]
    generate: bool,

    #[arg(long, default_value = "9")]
    generate_size: usize,

    #[arg(long, default_value = "3")]
    generate_grid_width: usize,

    #[arg(long, default_value = "3")]
    generate_grid_height: usize,

    #[arg(long, default_value = "123456789")]
    generate_charset: String,

    #[arg(long, default_value = "20")]
    generate_max_prune_seconds: u64,

    #[arg(long, default_value = "1")]
    generate_count: usize,

    #[arg(long, default_value = None)]
    solve_strategy: Option<String>,
}

fn operation_solve(file: std::path::PathBuf, solve_strategy: Option<String>) -> io::Result<()> {
    let contents = fs::read_to_string(file)?;

    println!("File contents:\n{}", contents);

    let mut sudoku = helpers::parse(contents)?;

    println!("{}", &sudoku.to_string());

    let start = Instant::now();
    let solved = solvers::solve(&mut sudoku, solve_strategy.as_deref());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("Solved: {}", solved);
    println!("{}", &sudoku.to_string());
    Ok(())
}

fn operation_generate(generator: generator::Generator, count: usize) {
    if !generator.validate_generator() {
        return ();
    }
    for _i in 0..count {
        println!("Generate solution...");
        let golden;
        let start1 = Instant::now();
        let maybe_golden = generator::generate_golden(&generator);
        println!("Time elapsed: {:?}", start1.elapsed());
        if let Some(g) = maybe_golden {
            golden = g;
        } else {
            println!("Error: Failure generating solution...");
            return ();
        }
        println!("Solution:");
        println!("{}", golden.to_string());
        println!("Generating challenge...");
        let start2 = Instant::now();
        let result = generator::generate_challenge(&generator, &golden);
        println!("Time elapsed: {:?}", start2.elapsed());
        match result {
            None => println!("Generating sudoku failed!"),
            Some(challenge) => {
                println!("Challenge:");
                println!("{}", challenge.to_string());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut unused = true;

    if args.solve {
        unused = false;
        let strategy = args.solve_strategy;
        match args.file {
            Some(file) => operation_solve(file, strategy)?,
            None => {
                println!("Error: --file must be specified when using --solve");
            }
        }
    }
    if args.generate {
        unused = false;
        let generator = generator::Generator {
            dimensions: args.generate_size,
            grid_width: args.generate_grid_width,
            grid_height: args.generate_grid_height,
            charset: args.generate_charset,
            max_prune_seconds: args.generate_max_prune_seconds,
        };
        operation_generate(generator, args.generate_count);
    }
    if unused {
        println!("Error: No operation? -h for help, also try --solve, --generate");
    }
    Ok(())
}
