use std::collections::HashMap;
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

fn _sort_moves(unsorted: Vec<Moves>) -> Vec<Moves> {
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

//setify...
fn uniq(v: Vec<String>) -> Vec<String> {
    let mut vv: Vec<String> = Vec::new();
    for s in v {
        if vv.contains(&s) {
            continue;
        }
        vv.push(s);
    }
    vv
}

struct Permuter {
    original: String,
    emitted: Vec<String>,
    first: bool,
    permutation: usize,
}

fn reduce_soup(soup: String, index: usize) -> (String, char) {
    let mut s = String::from("");
    let mut cc = '#';
    for (i, c) in soup.chars().enumerate() {
        if i == index {
            cc = c;
            continue;
        }
        s = s + &c.to_string();
    }
    return (s, cc);
}
impl Permuter {
    fn new(s: String) -> Permuter {
        Permuter {
            original: s,
            emitted: Vec::new(),
            first: true,
            permutation: 0,
        }
    }
    fn permutation_to_string(&self) -> Option<String> {
        let mut p = self.permutation;
        let mut radix = self.original.len();
        let mut soup = self.original.clone();
        let mut s = String::from("");
        for _i in 0..self.original.len() {
            let index = p % radix;
            p = p / radix;
            let (reduced_soup, c) = reduce_soup(soup, index);
            s = s + &c.to_string();
            soup = reduced_soup;
            radix = radix - 1;
        }
        if p == 0 {
            return Some(s);
        }
        None
    }
    fn next(&mut self) -> Option<String> {
        if self.first {
            self.first = false;
            self.permutation = 0;
            self.emitted.push(self.original.clone());
            return Some(self.original.clone());
        }
        loop {
            self.permutation = self.permutation + 1;
            let permuted = self.permutation_to_string();
            match permuted {
                Some(s) => {
                    if self.emitted.contains(&s) {
                        continue;
                    }
                    self.emitted.push(s.clone());
                    return Some(s);
                }
                None => return None,
            }
        }
    }
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
    fn backtrack_is_solved(&self) -> bool {
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                if '_' == self.board[row][col] {
                    return false;
                }
            }
        }
        return true;
    }
    fn valid_chars_row(&self, row: usize) -> String {
        let mut chars = (&"0123456789ABCDEF"[..self.dimensions]).to_string();
        for c in 0..self.dimensions {
            chars = chars.replace(self.board[row][c], "");
        }
        return chars;
    }
    fn valid_chars_col(&self, col: usize) -> String {
        let mut chars = (&"0123456789ABCDEF"[..self.dimensions]).to_string();
        for r in 0..self.dimensions {
            chars = chars.replace(self.board[r][col], "");
        }
        return chars;
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
    fn backtrack_solve(&mut self) -> bool {
        if self.backtrack_is_solved() {
            return true;
        }
        match self.possible_moves() {
            None => false,
            Some(moves_unsorted) => {
                //let moves = sort_moves(moves_unsorted);
                let moves = moves_unsorted;
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

    fn permutation_solve(&mut self) -> bool {
        let mut map: HashMap<String, usize> = HashMap::new();
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                let number = self.board[row][col];
                if '_' == number {
                    continue;
                }
                let number_s = number.to_string();
                let mut count: usize = match map.get(&number_s) {
                    Some(value) => *value,
                    None => 0,
                };
                count = count + 1;
                map.insert(number_s, count);
            }
        }
        /*
        let valid_chars = &"0123456789ABCDEF"[..self.dimensions];
        let mut unplaced: String = "".to_string();
        for c in valid_chars.chars() {
            let number_s = c.to_string();
            let count = map.get(&number_s).copied().unwrap_or(0);
            for _i in 0..(self.dimensions - count) {
                unplaced = unplaced + &c.to_string();
            }
        }
        println!("unplaced: {}", unplaced);
        */

        let board = self.board.clone();
        for row in 0..self.dimensions {
            let valid_row = self.valid_chars_row(row);
            println!("row {} valid characters: {}", row, valid_row);
            let mut permutator = Permuter::new(valid_row);
            'outer: while let Some(s) = permutator.next() {
                //println!("{}", s);
                //clean up any dirt
                for r in row..self.dimensions {
                    for c in 0..self.dimensions {
                        self.board[r][c] = board[r][c];
                    }
                }
                let mut i = 0;
                for col in 0..self.dimensions {
                    if '_' != self.board[row][col] {
                        continue;
                    }
                    let valid_col = self.valid_chars_col(col);
                    println!("row {} col {} valid characters: {}", row, col, valid_col);
                    match s.chars().nth(i) {
                        Some(c) => {
                            if !valid_col.contains(c) {
                                continue 'outer;
                            }
                            self.board[row][col] = c;
                            i = i + 1;
                            println!("row {} col {} {}", row, col, c);
                        }
                        None => {
                            println!("Error too few characters from permutation?");
                            break 'outer;
                        }
                    }
                }
            }
            for c in 0..self.dimensions {
                if self.board[row][c] == '_' {
                    break;
                }
            }
        }
        return self.backtrack_is_solved();
    }

    fn solve(&mut self) -> bool {
        return self.permutation_solve();
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

fn debug() {
    debug1();
    debug2();
}

fn debug1() {
    let mut p = Permuter::new("abcde".to_string());
    while let Some(s) = p.next() {
        println!("debug: {}", s);
    }
}
fn debug2() {
    let mut p = Permuter::new("aab".to_string());
    while let Some(s) = p.next() {
        println!("debug: {}", s);
    }
}

fn main() -> io::Result<()> {
    debug();
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
    sudoku.fill(data.clone());
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
