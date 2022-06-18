pub struct Number<T> {
    value: T,
}

pub trait Add<T> {
    fn add(&self, other: T) -> T;
}

impl Add<Number<u32>> for Number<u32> {
    fn add(&self, other: Number<u32>) -> Number<u32> {
        Number {
            value: other.value + self.value,
        }
    }
}

impl Add<Number<u64>> for Number<u32> {
    fn add(&self, other: Number<u64>) -> Number<u64> {
        Number {
            value: other.value + (self.value as u64),
        }
    }
}

pub fn demo() {
    let number1 = Number { value: 5_u32 };
    let number2 = Number { value: 10_u32 };
    let number3 = Number { value: 20_u64 };
    let result = number1.add(number2);
    let result2 = number1.add(number3);
}
