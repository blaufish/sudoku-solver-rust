pub struct Sudoku {
    pub board: [[u32; 16]; 16],
    pub dimensions: usize,
    subsquare_height: usize,
    subsquare_width: usize,
    character_set: String,
}

impl Sudoku {
    pub fn new(
        dimensions: usize,
        subsquare_height: usize,
        subsquare_width: usize,
        character_set: String,
    ) -> Sudoku {
        Sudoku {
            board: [[0; 16]; 16],
            dimensions: dimensions,
            subsquare_height: subsquare_height,
            subsquare_width: subsquare_width,
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
    pub fn utilized_subsuqare(&self, row: usize, col: usize) -> u32 {
        let mut binary: u32 = 0;
        let start_row: usize = (row / self.subsquare_height) * self.subsquare_height;
        let start_col: usize = (col / self.subsquare_width) * self.subsquare_width;
        let end_row = start_row + self.subsquare_height;
        let end_col = start_col + self.subsquare_width;

        for r in start_row..end_row {
            for c in start_col..end_col {
                binary = binary | self.board[r][c];
            }
        }
        return binary;
    }
}
