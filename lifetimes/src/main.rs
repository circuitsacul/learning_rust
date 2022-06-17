fn main() {
    let lg = {
        let a = 1;
        let b = 2;
        let lg = largest(&a, &b);
        *lg
    };
    println!("{}", lg);
}

fn largest<'a>(a: &'a i64, b: &'a i64) -> &'a i64 {
    if a > b {
        a
    } else {
        b
    }
}
