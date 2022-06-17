use std::io::Write;

pub fn demo_strings() {
    println!("Strings:");

    let s = "hello, world".to_string();
    println!("All types that implement Display have .to_string(): {}", s);

    // modifying strings
    let mut s = "hello".to_string();
    s.push_str(", world");
    println!("{}", s);

    // concatenating strings
    let s1 = "hello".to_string();
    let s2 = ", world".to_string();
    let s3 = s1 + &s2; // s1 no longer valid
    println!("{}", s3);

    // for more complex concatenation
    let s1 = "hello";
    let s2 = ", ".to_string();
    let s3 = &"world".to_string();

    let final_str = format!("{}{}{}", s1, s2, s3);
    println!("{}", final_str);

    // iterating strings
    let s = "hello, world".to_string();
    for char in s.chars() {
        print!("{}", char);
    }
    std::io::stdout().flush().unwrap();
    for b in s.bytes() {
        print!(" {} ", b);
    }
    std::io::stdout().flush().unwrap();
}
