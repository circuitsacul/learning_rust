use crate::board::Board;
use crate::character::Character;
use std::io;
use std::io::Write;

pub mod board;
pub mod character;

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
        get_user_choice(turn, board)
    } else {
        (row, col)
    }
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
        get_numeric_input(prompt, board)
    } else {
        inp
    }
}
