trait Wizard {
    fn fly(&self) -> &'static str;
}

trait Pilot {
    fn fly(&self) -> &'static str;
}

struct Human;

impl Human {
    fn fly(&self) -> &'static str {
        "*waving arms furiously*"
    }
}

impl Wizard for Human {
    fn fly(&self) -> &'static str {
        "*levitates into the air*"
    }
}

impl Pilot for Human {
    fn fly(&self) -> &'static str {
        "\"This is your pilot speaking\""
    }
}

pub fn demo() {
    let person = Human {};
    println!("{}", person.fly());
    println!("{}", Wizard::fly(&person));
    println!("{}", Pilot::fly(&person));
    takes_wizard(&person);
}

fn takes_wizard<T: Wizard>(person: &T) {
    println!("{}", person.fly());
}
