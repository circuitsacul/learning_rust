enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

use crate::cons_box::ConsList::{Cons, Nil};

pub fn demo() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
