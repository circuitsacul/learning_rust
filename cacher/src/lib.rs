use std::{collections::HashMap, hash::Hash};


pub struct Cacher<Func, In, Out>
    where
        In: Hash + Clone + Eq,
        Out: Hash + Clone + Eq,
        Func: Fn(&In) -> Out,
{
    pub callable: Func,
    pub cached: HashMap<In, Out>,
}

impl<Func, In, Out> Cacher<Func, In, Out>
    where
        In: Hash + Clone + Eq,
        Out: Hash + Clone + Eq,
        Func: Fn(&In) -> Out,
{
    pub fn value(&mut self, input: In) -> Out {
        match self.cached.get(&input) {
            Some(value) => value.clone(),
            None => {
                println!("Not cached...");
                let result = (self.callable)(&input);
                self.cached.insert(input.clone(), result.clone());
                result
            }
        }
    }
}
