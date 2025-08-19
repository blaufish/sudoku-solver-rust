use std::fs;
use std::io;
use std::io::Error;
use std::io::ErrorKind;

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

    let mut height = 0;
    let mut width = 0;
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
    validate_chars(width, data)?;

    Ok(())
}

//fn main() {
//    println!("Hello, world!");
//}
