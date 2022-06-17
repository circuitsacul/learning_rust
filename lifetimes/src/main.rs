fn main() {
    let lg = {
        let a = 1;
        let b = 2;
        let lg = largest(&a, &b);
        *lg
    };
    println!("{}", lg);

    static_only("hello, world!");
    // static_only("hello, world!".to_string().as_str())  // doesn't have a static lifetime
}

fn largest<'a>(a: &'a i64, b: &'a i64) -> &'a i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn static_only(string: &'static str) -> &'static str {
    // 'static lifetime refers to variables hardcoded into the program
    // itself, and live for the entire lifetime of the program itself.
    string
}
