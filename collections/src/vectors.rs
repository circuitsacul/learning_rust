pub fn demo_vector() {
    creating_and_updating_vectors();
    vector_indexing();
    referencing();
    iterating();
}

fn creating_and_updating_vectors() {
    println!("Creating+Updating vectors;");

    let vec1 = Vec::<i32>::new();
    println!("{:?}", vec1);
    let vec2: Vec<i32> = Vec::new();
    println!("{:?}", vec2);
    let mut vec3 = vec![1, 2, 3];
    println!("{:?}", vec3);

    drop(vec1);
    drop(vec2);

    vec3.push(4);
    vec3.extend(vec![5, 6, 7]);
    println!("pushed 4, extended with 5, 6, 7: {:?}", vec3);
}

fn vector_indexing() {
    println!("Indexing vectors:");

    let vector = vec![1, 2, 3];
    println!("Have vector {:?}", vector);

    let first = &vector[0];
    println!("First element: {}", first);

    match vector.get(1) {
        Some(item) => println!("Second element is {}", item),
        None => println!("Second element is nonexistent."),
    };
}

fn referencing() {
    println!("Vector references");

    let mut vector = vec![1, 2, 3];
    let first = &vector[0];
    // vector.push(4); // causes compile error
    println!("{}", first);
}

fn iterating() {
    println!("Iterating a vector");

    let mut vector = vec![1, 2, 3];
    println!("Vector {:?}", vector);
    for int in &mut vector {
        *int *= 10;
    }

    println!("Vector {:?}", vector);
}
