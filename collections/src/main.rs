use crate::vectors::demo_vector;
use crate::strings::demo_strings;
use crate::hashmaps::demo_hashmaps;

mod vectors;
mod strings;
mod hashmaps;

fn main() {
    demo_vector();
    demo_strings();
    demo_hashmaps();
}
