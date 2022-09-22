use num_bigint::{BigUint, ToBigUint};
use std::time::Instant;


fn fib(n: u32) -> BigUint {
    if n == 1 || n == 2 {
        return 1_i32.to_biguint().unwrap();
    }

    let mut last = 1_i32.to_biguint().unwrap();
    let mut curr = 1_i32.to_biguint().unwrap();

    for _ in 0..(n - 2) {
        let result = last + &curr;
        last = curr;
        curr = result;
    }

    curr
}

fn main() {
    let now = Instant::now();
    println!("{}", fib(1_000_000));
    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
