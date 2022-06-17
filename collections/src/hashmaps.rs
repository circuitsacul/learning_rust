use std::collections::HashMap;

pub fn demo_hashmaps() {
    println!("Hashmaps:");

    let mut map = HashMap::new();
    map.insert("hello", "goodbye");
    map.insert("goodbye", "hello");
    println!("{:#?}", map);

    let words = vec!["one", "two", "three"];
    let numbers = vec![1, 2, 3];

    let mut map: HashMap<_, _> = words.into_iter().zip(numbers.into_iter()).collect();
    println!("{:#?}", map);

    println!("Iterating...");
    for (k, v) in &map {
        println!("{}, {}", k, v);
    };

    println!("one={}", map["one"]);
    // map["one"] can panic
    // map.get("one") returns Option<T>

    // inserts will ovewrite existing values
    // do this to only insert if it exists
    map.entry("one").or_insert(1);
    // this returns the value that already existed or was inserted.

    // map.remove exists too, for deleting keys.
    // map.remove_entry also
}
