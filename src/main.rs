use std::fs;
use std::io;
use std::time::Instant;

use clap::Parser;

mod generator;
mod helpers;
mod solvers;
mod sudoku;

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

    #[arg(long, default_value = "2")]
    generate_internal_max_entries: usize,

    #[arg(long, default_value = "2")]
    generate_internal_picks_per_solve: usize,

    #[arg(long, default_value = "3")]
    generate_internal_initial_randomized_cells: usize,

    #[arg(long, default_value = "6")]
    generate_internal_kickstart_cells: usize,

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
        let start = Instant::now();
        let result = generator::generate(&generator);
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        match result {
            None => println!("Generating sudoku failed!"),
            Some((challenge, solution)) => {
                println!("Challenge:");
                println!("{}", challenge.to_string());
                println!("Solution:");
                println!("{}", solution.to_string());
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
            threshold: args.generate_internal_max_entries,
            picks_per_solve: args.generate_internal_picks_per_solve,
            initial_randomized_cells: args.generate_internal_initial_randomized_cells,
            kickstart_cells: args.generate_internal_kickstart_cells,
        };
        operation_generate(generator, args.generate_count);
    }
    if unused {
        println!("Error: No operation? -h for help, also try --solve, --generate");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::helpers;
    use crate::solvers;
    fn process(testvector: String) -> String {
        let mut sudoku;
        match helpers::parse(testvector) {
            Ok(s) => sudoku = s,
            Err(_) => return String::from(""),
        }
        let _ = solvers::solve(&mut sudoku, None);
        sudoku.to_string()
    }

    #[test]
    fn test_unicode() {
        let vector = "__💩 ___ __🍅\n_🌈🍅 _💩_ ___\n___ 🍅__ 🍕💩🌈\n\n\
                      🎆🎌🌈 🐡_🐙 💩_🍕\n🍅🏠🍕 💩🎌🎆 __🐙\n_💩_ 🌈_🍕 🎌__\n\n\
                      ___ 🎌__ 🏠_💩\n_🍕_ _🐙_ ___\n__🎆 ___ __🐡";
        let expected = "🎌🐡💩 🐙🍕🌈 🎆🏠🍅\n🍕🌈🍅 🎆💩🏠 🐡🐙🎌\n🐙🎆🏠 🍅🐡🎌 🍕💩🌈\n\n\
                        🎆🎌🌈 🐡🏠🐙 💩🍅🍕\n🍅🏠🍕 💩🎌🎆 🌈🐡🐙\n🐡💩🐙 🌈🍅🍕 🎌🎆🏠\n\n\
                        🌈🐙🐡 🎌🎆🍅 🏠🍕💩\n💩🍕🎌 🏠🐙🐡 🍅🌈🎆\n🏠🍅🎆 🍕🌈💩 🐙🎌🐡";
        let actual = process(vector.to_string());
        assert_eq!(expected, actual);
    }
}
