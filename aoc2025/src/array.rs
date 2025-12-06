use std::{iter::Sum, ops::Mul};

pub trait ArrayExt<T>: Iterator<Item = T> {
    fn as_number(&mut self) -> T
    where
        Self: DoubleEndedIterator,
        T: Mul<u64>,
        T: Sum<<T as Mul<u64>>::Output>, {
        self.into_iter()
            .rev()
            .enumerate()
            .map(|(i, val)| {
                let multiplier = 10u64.pow(i as _);

                return val * multiplier;
            })
            .sum()
    }
}

impl<T, U: Iterator<Item = T>> ArrayExt<T> for U {}
