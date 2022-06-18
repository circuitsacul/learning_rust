enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

use std::ops::Deref;

use crate::ConsList::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let mb = MyBox::new(x);
    assert_eq!(x, *mb);
}
