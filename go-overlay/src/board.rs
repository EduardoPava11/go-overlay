#[derive(Clone, Copy, PartialEq)]
pub enum Stone {
    Empty,
    Black,
    White,
}

#[derive(Clone)]
pub struct Board {
    pub grid: [[Stone; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Stone::Empty; 9]; 9],
        }
    }

    pub fn place_stone(&mut self, row: usize, col: usize, stone: Stone) {
        if row < 9 && col < 9 {
            self.grid[row][col] = stone;
        }
    }
}