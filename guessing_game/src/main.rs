use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let answer = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please pass a positive number.");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&answer) {
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Too big."),
            Ordering::Less => println!("Too small."),
        }
    }
}
