//use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Error;
use std::io::ErrorKind;

mod sudoku;
use crate::sudoku::Sudoku;

fn solve(sudoku: &mut Sudoku) -> bool {
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
    /*
    println!(
        "sudoku.board[{}][{}] utilized: {} {} {} {}",
        row, col, utilized, utilized_row, utilized_col, utilized_subsuqare
    );
    */
    for i in 0..sudoku.dimensions {
        let binary: u32 = 1 << i;
        if binary & utilized != 0 {
            /*
            println!(
                "sudoku.board[{}][{}] Skip: {} {}",
                row, col, binary, utilized
            );
            */
            continue;
        }
        sudoku.board[row][col] = binary;
        //println!("sudoku.board[{}][{}] = {}", row, col, binary);
        let recursive_solved = solve(sudoku);
        if recursive_solved {
            return true;
        } else {
            sudoku.board[row][col] = 0;
        }
    }
    false
}

fn printsudoku(sudoku: &Sudoku) {
    for row in 0..sudoku.dimensions {
        for col in 0..sudoku.dimensions {
            print!("{}", sudoku.get_c(row, col));
            if (col % sudoku.subsquare_width) == sudoku.subsquare_width - 1 {
                print!(" ");
            }
        }
        println!("");
        if (row % sudoku.subsquare_height) == sudoku.subsquare_height - 1 {
            println!("");
        }
    }
    println!("");
}

fn charset_from_sudoku_vector(width: usize, v: Vec<String>) -> Option<String> {
    let mut charset = String::from("");
    for s in v {
        for c in s.chars() {
            if c == '_' {
                continue;
            }
            if c == '.' {
                continue;
            }
            if charset.contains(c) {
                continue;
            }
            charset = charset + &c.to_string();
        }
    }
    if charset.len() > width {
        return None;
    }
    if charset.len() == width {
        return Some(charset);
    }
    let valid_chars_s = "0123456789ABCDEF";
    for c in valid_chars_s.chars() {
        if !charset.contains(c) {
            charset = charset + &c.to_string();
            if charset.len() == width {
                return Some(charset);
            }
        }
    }
    return None;
}

fn main() -> io::Result<()> {
    let file_path = "challenge.txt";

    let contents = fs::read_to_string(file_path)?;

    println!("File contents:\n{}", contents);

    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut data: Vec<String> = Vec::new();

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let processed = line.trim().replace(" ", "");
        let length = processed.len();
        if length == 0 {
            continue;
        }
        if width == 0 {
            width = length;
        } else {
            if width != length {
                return Err(Error::new(ErrorKind::Other, "Inconsistent line length"));
            }
        }
        println!("Line: <{}>", processed);
        data.push(processed);
        height = height + 1;
    }
    println!("H/W: {} {}", height, width);
    if width != height {
        return Err(Error::new(ErrorKind::Other, "Width and Height missmatch"));
    }
    //validate_chars(width, data.clone())?;

    let subsquare_height;
    let subsquare_width;

    match width {
        1 => {
            subsquare_height = 1;
            subsquare_width = 1;
        }
        2 => {
            subsquare_height = 2;
            subsquare_width = 2;
        }
        3 => {
            subsquare_height = 3;
            subsquare_width = 3;
        }
        4 => {
            subsquare_height = 2;
            subsquare_width = 2;
        }
        6 => {
            subsquare_height = 3;
            subsquare_width = 2;
        }
        9 => {
            subsquare_height = 3;
            subsquare_width = 3;
        }
        16 => {
            subsquare_height = 4;
            subsquare_width = 4;
        }
        _ => {
            return Err(Error::new(ErrorKind::Other, "Unknown sudoku type"));
        }
    }

    let charset_opt = charset_from_sudoku_vector(width, data.clone());
    let charset: String;
    match charset_opt {
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "Cannot generate character set!",
            ));
        }
        Some(cs) => charset = cs,
    }
    println!("subsquare_height: {}", subsquare_height);
    println!("subsquare_width: {}", subsquare_width);
    println!("charset: {}", charset);

    let mut sudoku: Sudoku = Sudoku::new(width, subsquare_height, subsquare_width, charset);
    sudoku.fill(data.clone());

    printsudoku(&sudoku);
    /*
    for row in 0..height {
        for col in 0..width {
            print!("{}", sudoku.get_c(row, col));
        }
        println!("");
    }
    println!("");
    */
    let solved = solve(&mut sudoku);
    println!("Solved: {}", solved);
    printsudoku(&sudoku);
    /*
    for row in 0..height {
        for col in 0..width {
            print!("{}", sudoku.get_c(row, col));
        }
        println!("");
    }
    println!("");
    */
    Ok(())
}
