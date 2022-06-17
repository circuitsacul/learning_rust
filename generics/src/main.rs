struct Point<X: PartialOrd, Y: PartialOrd> {
    x: X,
    y: Y,
}

impl<X: PartialOrd, Y: PartialOrd> Point<X, Y> {
    fn can_fit(&self, other: &Point<X, Y>) -> bool {
        other.x < self.x && other.y < self.y
    }
}


fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    println!("Hello, world!");

    let list = vec![1, 2, 3];
    let largest = find_largest(&list);
    println!("The largest of {:?} is {}", list, largest);
}
