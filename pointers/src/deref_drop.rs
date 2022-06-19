use std::ops::Deref;

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

struct CustomCleanup;

impl Drop for CustomCleanup {
    fn drop(&mut self) {
        println!("Drop called.");
    }
}

pub fn demo() {
    // deref trait
    let x = 5;
    let mb = MyBox::new(x);
    assert_eq!(x, *mb);

    // Drop trait
    println!("Making with drop trait");
    {
        let x = CustomCleanup{};
        println!("Made!, lowering scope.");
    }
    println!("Outside scope.");

}
