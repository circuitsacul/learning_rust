use std::io;
use std::io::Write;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Character {
    X,
    O,
    E,
}

#[derive(Debug)]
struct Board([Character; 3], [Character; 3], [Character; 3]);

impl Board {
    fn make_row() -> [Character; 3] {
        [Character::E, Character::E, Character::E]
    }

    fn new() -> Board {
        Board(
            Board::make_row(),
            Board::make_row(),
            Board::make_row(),
        )
    }

    fn set(&mut self, row: usize, col: usize, val: &Character) {
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

    fn get(&self, row: usize, col: usize) -> &Character {
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

    fn print_board(&self) {
        fn print_char(char: &Character) {
            if char == &Character::X {
                print!(" X ");
            } else if char == &Character::O {
                print!(" O ");
            } else if char == &Character::E {
                print!("   ");
            }
        }

        for row in [&self.0, &self.1, &self.2] {
            for char in row {
                print!("|");
                print_char(char);
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

fn main() {
    let board = Board::new();

    main_loop(board);
}

fn main_loop(mut board: Board) {
    let mut turn = Character::X;
    loop {
        board.print_board();

        let (row, col) = get_user_choice(&turn, &board);
        board.set(row, col, &turn);

        turn = match turn {
            Character::X => Character::O,
            Character::O => Character::X,
            Character::E => panic!("Turn should never be Character::E."),
        };
    }
}

fn get_user_choice(turn: &Character, board: &Board) -> (usize, usize) {
    println!("It is player {:?}'s turn.", turn);
    let row = get_numeric_input("Please input the row: ", board);
    let col = get_numeric_input("Please input the column: ", board);

    if board.get(row, col) != &Character::E {
        println!("That spot is already occupied!");
        return get_user_choice(turn, board);
    }

    (row, col)
}

fn get_numeric_input(prompt: &str, board: &Board) -> usize {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to write line");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read line");

    let inp = inp.trim().parse::<usize>();
    let inp = match inp {
        Ok(val) => val,
        Err(_) => {
            println!("Please input a valid posistive integer.");
            return get_numeric_input(prompt, board);
        }
    };

    if inp < 1 || inp > 3 {
        println!("Please input a number between 1 and 3 inclusive.");
        return get_numeric_input(prompt, board);
    }

    inp
}
