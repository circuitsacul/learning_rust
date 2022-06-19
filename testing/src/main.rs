use std::io;

mod ambig_trait;

fn main() {
    println!("Hello, world!");
    demo_result();
    demo_tup();
    demo_array();
    demo_let_as_fn_ret_type();
    demo_if_expr();

    ambig_trait::demo();
}

// if as an expression
fn demo_if_expr() {
    let condition = true;
    let var = if condition { 3 } else { 6 };
    println!("condition was {} so value is {}", condition, var);
}

// passing let to parent, function return types, as keyword
fn demo_let_as_fn_ret_type() {
    fn five() -> u8 {
        return 5;
    }
    let y = {
        let x = five() as u32;
        x + 1
    };

    println!("{}", y);
}

// arrays
fn demo_array() {
    let zeros = [0; 2];
    let [x, y] = zeros;

    println!("Destructured array into {}, {}", x, y);
}

// tuples and destructuring
fn demo_tup() {
    println!("Demo of tuples:");
    let my_tup: (u32, u8, String) = (5, 5, String::from("hi"));
    let (x, y, z) = &my_tup;
    println!("Destructured into {x} {y} {z}", x = x, y = y, z = z);
    println!("Dot-notation for indexing my_tup.1={}", my_tup.1);
}

// Result
fn demo_result() {
    println!("Demo of Result:");
    let ret = u32_or_err();
    match ret {
        Ok(num) => println!("Returned {}", num),
        Err(why) => println!("Raised an error! {}", why),
    }
}

fn u32_or_err() -> Result<u32, String> {
    println!("Please input either 'error' or 'pass'.");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read line");

    match inp.as_str().trim() {
        "error" => Err("Some error!".to_string()),
        "pass" => Ok(50),
        _ => {
            println!("Invalid input.");
            u32_or_err()
        }
    }
}
