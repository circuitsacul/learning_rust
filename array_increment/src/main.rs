use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut arr = Vec::new();
    for val in 0..10_000_000 {
        arr.push(val);
    }

    for val in arr.iter_mut() {
        *val += 1;
    }

    let elapsed = start.elapsed();
    println!("{}", elapsed.as_secs_f64());
}
