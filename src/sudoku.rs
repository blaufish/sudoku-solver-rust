pub const MAX_GRID_DIMENSIONS: usize = 5;
pub const MAX_DIMENSIONS: usize = MAX_GRID_DIMENSIONS * MAX_GRID_DIMENSIONS;

#[derive(Clone)]
pub struct Sudoku {
    pub board: [[u32; MAX_DIMENSIONS]; MAX_DIMENSIONS],
    pub dimensions: usize,
    pub grid_height: usize,
    pub grid_width: usize,
    pub character_set: String,
}

impl Sudoku {
    pub fn new(
        dimensions: usize,
        grid_height: usize,
        grid_width: usize,
        character_set: String,
    ) -> Sudoku {
        Sudoku {
            board: [[0; MAX_DIMENSIONS]; MAX_DIMENSIONS],
            dimensions: dimensions,
            grid_height: grid_height,
            grid_width: grid_width,
            character_set: character_set,
        }
    }
    fn set_c(&mut self, row: usize, col: usize, c: char) {
        let mut binary: u32 = 1;
        for cc in self.character_set.chars() {
            if cc == c {
                self.board[row][col] = binary;
                break;
            }
            binary = binary << 1;
        }
    }
    pub fn get_c(&self, row: usize, col: usize) -> char {
        let val = self.board[row][col];
        let mut binary: u32 = 1;
        for cc in self.character_set.chars() {
            if val == binary {
                return cc;
            }
            binary = binary << 1;
        }
        return '_';
    }
    pub fn fill(&mut self, v: Vec<String>) {
        for (row, s) in v.iter().enumerate() {
            if row >= self.dimensions {
                continue;
            }
            for (col, c) in s.chars().enumerate() {
                if col >= self.dimensions {
                    continue;
                }
                self.set_c(row, col, c);
            }
        }
    }
    pub fn utilized_row(&self, row: usize) -> u32 {
        let mut binary: u32 = 0;
        for col in 0..self.dimensions {
            binary = binary | self.board[row][col];
        }
        return binary;
    }
    pub fn utilized_col(&self, col: usize) -> u32 {
        let mut binary: u32 = 0;
        for row in 0..self.dimensions {
            binary = binary | self.board[row][col];
        }
        return binary;
    }
    pub fn utilized_grid(&self, row: usize, col: usize) -> u32 {
        let mut binary: u32 = 0;
        let start_row: usize = (row / self.grid_height) * self.grid_height;
        let start_col: usize = (col / self.grid_width) * self.grid_width;
        let end_row = start_row + self.grid_height;
        let end_col = start_col + self.grid_width;

        for r in start_row..end_row {
            for c in start_col..end_col {
                binary = binary | self.board[r][c];
            }
        }
        return binary;
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("");
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                s += &String::from(self.get_c(row, col));
                if (col % self.grid_width) != self.grid_width - 1 {
                    continue;
                }
                if col == self.dimensions - 1 {
                    continue;
                }
                s = s + " ";
            }

            if row == self.dimensions - 1 {
                break;
            }
            s += "\n";
            if (row % self.grid_height) != self.grid_height - 1 {
                continue;
            }
            s += "\n";
        }
        s
    }

    pub fn validate(&self) -> (bool, Vec<(usize, usize, String)>) {
        let mut errors: Vec<(usize, usize, String)> = Vec::new();
        let mut valid = true;
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                if self.board[row][col] == 0 {
                    valid = false;
                    errors.push((row, col, "unset".to_string()));
                }
            }
        }
        for row in 0..self.dimensions {
            let mut utilized: u32 = 0;
            for col in 0..self.dimensions {
                if self.board[row][col] == 0 {
                    continue;
                }
                if self.board[row][col] & utilized != 0 {
                    valid = false;
                    errors.push((row, col, "duplicated in row".to_string()));
                }
                utilized = utilized | self.board[row][col];
            }
        }
        for col in 0..self.dimensions {
            let mut utilized: u32 = 0;
            for row in 0..self.dimensions {
                if self.board[row][col] == 0 {
                    continue;
                }
                if self.board[row][col] & utilized != 0 {
                    valid = false;
                    errors.push((row, col, "duplicated in col".to_string()));
                }
                utilized = utilized | self.board[row][col];
            }
        }
        for grid_h in 0..self.dimensions / self.grid_height {
            for grid_w in 0..self.dimensions / self.grid_width {
                let mut utilized: u32 = 0;
                for pre_row in 0..self.grid_height {
                    for pre_col in 0..self.grid_width {
                        let row = grid_h * self.grid_height + pre_row;
                        let col = grid_w * self.grid_width + pre_col;
                        if self.board[row][col] == 0 {
                            continue;
                        }
                        if self.board[row][col] & utilized != 0 {
                            valid = false;
                            errors.push((row, col, "duplicated in grid".to_string()));
                        }
                        utilized = utilized | self.board[row][col];
                    }
                }
            }
        }
        (valid, errors)
    }
}

fn _sudoku_equals(a: &Sudoku, b: &Sudoku) -> bool {
    if a.dimensions != b.dimensions {
        return false;
    }
    if a.grid_height != b.grid_height {
        return false;
    }
    if a.grid_width != b.grid_width {
        return false;
    }
    for row in 0..a.dimensions {
        for col in 0..a.dimensions {
            if a.board[row][col] != b.board[row][col] {
                return false;
            }
        }
    }
    true
}
