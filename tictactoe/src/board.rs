use crate::character::Character;

#[derive(Debug)]
pub struct Board([Character; 3], [Character; 3], [Character; 3]);

impl Board {
    fn make_row() -> [Character; 3] {
        [Character::E, Character::E, Character::E]
    }

    pub fn new() -> Board {
        Board(Board::make_row(), Board::make_row(), Board::make_row())
    }

    pub fn set(&mut self, row: usize, col: usize, val: &Character) {
        let val = val.to_owned();
        if row == 1 {
            self.0[col - 1] = val;
        } else if row == 2 {
            self.1[col - 1] = val;
        } else if row == 3 {
            self.2[col - 1] = val;
        } else {
            panic!("Received invalid row.");
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &Character {
        if row == 1 {
            return &self.0[col - 1];
        } else if row == 2 {
            return &self.1[col - 1];
        } else if row == 3 {
            return &self.2[col - 1];
        } else {
            panic!("Invalid row")
        }
    }

    pub fn print_board(&self) {
        for row in [&self.0, &self.1, &self.2] {
            for char in row {
                print!("|");
                char.print();
            }
            print!("|");
            print!("\n");
            for _ in 0..13 {
                print!("-");
            }
            print!("\n");
        }
    }
}
