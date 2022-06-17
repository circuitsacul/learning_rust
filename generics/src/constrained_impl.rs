struct Thing<T> {
    value: T
}

impl<T> Thing<T> {
    fn this_exists_for_everything(&self) {}
}

impl Thing<f32> {
    fn this_exists_for_f32(&self) {}
}

pub fn demo() {
    let thing_f32 = Thing{value:1_f32};
    let thing_int = Thing{value:1_i32};

    thing_f32.this_exists_for_everything();
    thing_f32.this_exists_for_f32();

    thing_int.this_exists_for_everything();
    // thing_int.this_exists_for_f32();
}
