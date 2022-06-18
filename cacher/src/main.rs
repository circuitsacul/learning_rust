use std::collections::HashMap;

use cacher::Cacher;


fn main() {
    let mut cacher = Cacher {
        callable: |input: &i64| -> i64 { input*10 },
        cached: HashMap::<i64, i64>::new(),
    };
    let x = cacher.value(3);
    let y = cacher.value(3);
    println!("got {}, {}", x, y);
}
