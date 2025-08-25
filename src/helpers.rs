use std::io;
use std::io::Error;
use std::io::ErrorKind;

use crate::sudoku;

pub fn parse(contents: String) -> io::Result<sudoku::Sudoku> {
    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut data: Vec<String> = Vec::new();

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let processed = line.trim().replace(" ", "");
        let length = processed.chars().count();
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
        data.push(processed);
        height = height + 1;
    }
    println!("H/W: {} {}", height, width);
    if width != height {
        return Err(Error::new(ErrorKind::Other, "Width and Height missmatch"));
    }

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
            subsquare_height = 2;
            subsquare_width = 3;
        }
        9 => {
            subsquare_height = 3;
            subsquare_width = 3;
        }
        12 => {
            subsquare_height = 3;
            subsquare_width = 4;
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
    let mut sudoku: sudoku::Sudoku =
        sudoku::Sudoku::new(width, subsquare_height, subsquare_width, charset);
    sudoku.fill(data.clone());
    Ok(sudoku)
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
    if charset.chars().count() > width {
        return None;
    }
    if charset.chars().count() == width {
        return Some(charset);
    }
    let valid_chars_s = "0123456789ABCDEF";
    for c in valid_chars_s.chars() {
        if !charset.contains(c) {
            charset = charset + &c.to_string();
            if charset.chars().count() == width {
                return Some(charset);
            }
        }
    }
    return None;
}

pub fn printsudoku(sudoku: &sudoku::Sudoku) {
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
