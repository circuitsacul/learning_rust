use std::ops::AddAssign;

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_list: Vec<_> = list.iter().map(|x| x + 1).collect();
    println!("{:?}\n{:?}", list, new_list);

    let value = 2;
    let divisible: Vec<_> = list.into_iter().filter(|x| x % value == 0).collect();
    println!("Divisible by {}: {:?}", value, divisible);

    let counter = Counter::new(0, 5, 1);
    let values = counter.into_iter().collect::<Vec<_>>();
    println!("0-5: {:?}", values);
}

struct Counter<T> {
    value: T,
    stop: T,
    jump: T,
}

impl<T> Counter<T> {
    fn new(value: T, stop: T, jump: T) -> Counter<T> {
        Counter { value, stop, jump }
    }
}

impl<T: AddAssign<T> + Eq + Clone> Iterator for Counter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == self.stop {
            None
        } else {
            self.value += self.jump.clone();
            Some(self.value.clone())
        }
    }
}
