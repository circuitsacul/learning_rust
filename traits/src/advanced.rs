#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Number<T: Copy + Clone + Eq> {
    value: T,
}

pub trait Add<T> {
    type Output;
    fn add(&self, other: T) -> Self::Output;
}

impl Add<Number<u32>> for Number<u32> {
    type Output = Number<u32>;
    fn add(&self, other: Number<u32>) -> Number<u32> {
        Number {
            value: other.value + self.value,
        }
    }
}

impl Add<Number<u64>> for Number<u32> {
    type Output = Number<u64>;
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
    let sum_result = sum(number1, number2);

    let result2 = number1.add(number3);
    let sum_result2 = sum(number1, number3);

    assert_eq!(result, sum_result);
    assert_eq!(result2, sum_result2);
}

fn sum<L, R, O>(left: L, right: R) -> O
    where
        L: Add<R, Output=O>
{
    left.add(right)
}
