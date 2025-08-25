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

    println!("{}", &sudoku.to_string());

    let start = Instant::now();
    let solved = solvers::solve(&mut sudoku, strategy);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("Solved: {}", solved);
    println!("{}", &sudoku.to_string());
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
