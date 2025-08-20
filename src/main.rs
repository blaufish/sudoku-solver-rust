use std::fs;
use std::io;
use std::io::Error;
use std::io::ErrorKind;

#[derive(Clone)]
struct Moves {
    row: usize,
    col: usize,
    chars: String,
}

fn sort_moves(unsorted: Vec<Moves>) -> Vec<Moves> {
    let mut sorted: Vec<Moves> = Vec::new();
    let mut max = 0;
    for v in unsorted.clone() {
        let len = v.chars.len();
        if len == 0 {
            // this path is f'ed up, abort early
            return sorted;
        }
        if len > max {
            max = len;
        }
    }
    for len in 0..(max + 1) {
        for v in unsorted.clone() {
            if v.chars.len() == len {
                sorted.push(v)
            }
        }
    }
    sorted
}

struct Sudoku {
    board: [[char; 16]; 16],
    dimensions: usize,
}

impl Sudoku {
    fn new(dimensions: usize) -> Sudoku {
        Sudoku {
            board: [['X'; 16]; 16],
            dimensions: dimensions,
        }
    }
    fn fill(&mut self, v: Vec<String>) {
        for (row, s) in v.iter().enumerate() {
            if row >= self.dimensions {
                continue;
            }
            for (col, c) in s.chars().enumerate() {
                if col >= self.dimensions {
                    continue;
                }
                self.board[row][col] = c;
            }
        }
    }
    fn is_solved(&self) -> bool {
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                if '_' == self.board[row][col] {
                    return false;
                }
            }
        }
        return true;
    }
    fn possible_moves(&self) -> Option<Vec<Moves>> {
        let mut v: Vec<Moves> = Vec::new();
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                if '_' != self.board[row][col] {
                    continue;
                }
                let mut m: Moves = Moves {
                    row: row,
                    col: col,
                    chars: (&"0123456789ABCDEF"[..self.dimensions]).to_string(),
                };
                for r in 0..self.dimensions {
                    m.chars = m.chars.replace(self.board[r][col], "");
                }
                for c in 0..self.dimensions {
                    m.chars = m.chars.replace(self.board[row][c], "");
                }
                if m.chars.len() == 0 {
                    return None;
                }
                v.push(m);
            }
        }
        Some(v)
    }
    fn solve(&mut self) -> bool {
        if self.is_solved() {
            return true;
        }
        match self.possible_moves() {
            None => false,
            Some(moves_unsorted) => {
                let moves = sort_moves(moves_unsorted);
                //println!("Moves: {}", moves.len());
                for m in moves {
                    for c in m.chars.chars() {
                        self.board[m.row][m.col] = c;
                        let solved = self.solve();
                        if solved {
                            return true;
                        }
                        self.board[m.row][m.col] = '_';
                    }
                }
                return false;
            }
        }
    }
}

fn validate_chars(hw: usize, v: Vec<String>) -> io::Result<()> {
    let valid_chars_s = &"_0123456789ABCDEF"[..(hw + 1)];
    //let valid_chars = valid_chars_s.chars();
    for s in v.iter() {
        for c in s.chars() {
            if !valid_chars_s.contains(c) {
                return Err(Error::new(ErrorKind::Other, "Illegal character"));
            }
        }
    }
    Ok(())
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
    validate_chars(width, data.clone())?;
    let mut sudoku: Sudoku = Sudoku::new(width);
    sudoku.fill(data);
    let solved = sudoku.solve();
    println!("solved: <{}>", solved);

    for row in 0..height {
        for col in 0..width {
            print!("{}", sudoku.board[row][col]);
        }
        println!("");
    }
    Ok(())
}

//fn main() {
//    println!("Hello, world!");
//}
