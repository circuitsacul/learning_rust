use core::fmt::Display;

fn main() {
    // types that implemente the Clone trait are on the stack and are
    // copied when re-assigning variables.
    // types that have the Clone trait include i<size>, u<size>, f<size>,
    // &str, tuple (assuming all sub-types also implement Clone)
    let x = 5;
    let y = x;
    println!("copied data on the stack; x={} and y={} are valid.", x, y);

    // types that are on the heap are shallow copied or "moved", resulting
    // in the original variable becoming invalid. These types implement the
    // Drop method, which is called when the owner goes out of scope.
    let x = String::from("hello");
    let y = x;
    println!("moved data on the heap; y={} is valid, x is not.", y);

    // some types can simply be cloned though
    let x = String::from("hello");
    let y = x.clone();
    println!("cloned data on the heap; x={} and y={} are valid.", x, y);

    // when calling functions, variables are either moved to the function
    // or are copied (same rules as above).
    let x = 5;
    makes_copy(x);
    println!("But x={} is still valid", x);

    let x = String::from("hello");
    steals_ownership(x);
    println!("x was stolen, no longer valid.");

    // function returns can also transfer ownership
    let x = String::from("hello");
    println!("I have x={}", x);
    let y = steals_and_gives_back(x);
    println!("x is invalid, but now have y={}", y);

    // rust allows for references, or borrows, to prevent needing to return
    // everything
    let x = String::from("hello");
    let length = calc_length(&x);
    println!("The length of {} is {}", x, length);

    // mutable borrows
    // theses allow a borrow to mutate the string, but there can only be one mutable
    // borrow at a time. If there is a mutable borrow, you also cannot have any
    // immutable borrows.
    let mut x = String::from("hello");
    add_world(&mut x);
    println!("{}", x);

    // string slices
    let my_str = String::from("hello, world");
    let first_word = get_first_word(&my_str);
    println!("The first word of {} is {}", my_str, first_word);
}

fn makes_copy<F: Copy + Display>(x: F) {
    println!("Copied and printing {}", x);
}

fn steals_ownership<F: Display>(x: F) {
    println!("Muwahahaha I have stolen {}", x);
}

fn steals_and_gives_back<F: Display>(x: F) -> F {
    println!("Stole x={} but giving it back", x);
    x
}

fn calc_length(x: &String) -> usize {
    return x.len();
}

fn add_world(x: &mut String) {
    x.push_str(", world!");
}

fn get_first_word(the_str: &str) -> &str {
    let as_bytes = the_str.as_bytes();
    for (i, &chr) in as_bytes.iter().enumerate() {
        if chr == b' ' {
            return &the_str[0..i];
        }
    }
    return the_str;
}
