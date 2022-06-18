fn main() {
    let y = 5;
    let plus_5 = |x| x + y;
    println!("10 + 5 = {}", plus_5(10));

    let x = "hello".to_string();
    let closure = move || x;
    // println!("{}", x);
    let x = closure();
    println!("{}", x);
}
