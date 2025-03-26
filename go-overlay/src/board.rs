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
    
    pub fn print_board(&self) {
        println!();
        print!("   ");
        for col in 0..9 {
            print!(" {:>2}", col);
        }
        println!();

        for (row_idx, row) in self.grid.iter().enumerate() {
            print!("{:>2} ", row_idx);
            for &stone in row {
                let symbol = match stone {
                    Stone::Black => "●",
                    Stone::White => "○",
                    Stone::Empty => "+",
                };
                print!("  {}", symbol);
            }
            println!();
        }
        println!("\n\n");
    }
}
