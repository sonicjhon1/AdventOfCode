#![feature(int_roundings)]

use aoc2025::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::fmt::Display;

const INPUT_TEST: &str = include_str!("2025_02_input_test.txt");
const INPUT_1: &str = include_str!("2025_02_input_1.txt");
const INPUT_2: &str = include_str!("2025_02_input_2.txt");

fn main() {
    init_tracing();

    {
        debug_assert!(!ProductID::from_string("55").unwrap().is_valid_part_1());
        debug_assert!(!ProductID::from_string("6464").unwrap().is_valid_part_1());
        debug_assert!(!ProductID::from_string("123123").unwrap().is_valid_part_1());
        debug_assert!(ProductID::from_string("0101").is_err());

        debug_assert!(
            !ProductID::from_string("12341234")
                .unwrap()
                .is_valid_part_2()
        );
        debug_assert!(
            !ProductID::from_string("123123123")
                .unwrap()
                .is_valid_part_2()
        );
        debug_assert!(
            !ProductID::from_string("1212121212")
                .unwrap()
                .is_valid_part_2()
        );
    }

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        debug_assert_eq!(part_1, 1227775554);
        debug_assert_eq!(part_2, 4174379265);
    }

    solution(INPUT_1);
    solution(INPUT_2);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    /// Sum of invalid ProductIDs (digit repeated twice)
    pub part_1: u64,
    /// Sum of invalid ProductIDs (digit repeated atleast twice)
    pub part_2: u64,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ProductIDRange {
    pub first: ProductID,
    pub last: ProductID,
}

impl IntoParallelIterator for ProductIDRange {
    type Iter = rayon::iter::Map<rayon::range_inclusive::Iter<u64>, fn(u64) -> ProductID>;

    type Item = ProductID;

    fn into_par_iter(self) -> Self::Iter {
        (self.first.0..=self.last.0).into_par_iter().map(ProductID)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deref, DerefMut)]
pub struct ProductID(pub u64);

impl ProductID {
    pub fn from_string(value: impl Display) -> Result<Self> {
        let value = value.to_string();

        if value.starts_with('0') {
            return Err("Invalid ID: Shouldn't start with 0".into());
        }

        let parsed_value = value.parse()?;

        Ok(Self(parsed_value))
    }

    fn is_valid_part_1(&self) -> bool {
        let text = self.to_string();
        let len = text.len();

        if len == 0 {
            return false;
        }

        if len.is_multiple_of(2) {
            let (left, right) = text.split_at(len / 2);

            if right.contains(left) {
                return false;
            }
        }

        return true;
    }

    fn is_valid_part_2(&self) -> bool {
        let text = self.to_string();
        let len = text.len();

        if len == 0 {
            return false;
        }

        return !(1..len).into_par_iter().any(|i| {
            let left = text.split_at(i).0;

            if !len.is_multiple_of(i) {
                return false
            }

            let n_repeat = len.div_euclid(i);
            let left_repeated = left.repeat(n_repeat);

            debug!(
                "Text {text}; len {len}; i {i}; n_repeat {n_repeat}; left {left}; left_repeated {left_repeated}",
            );

            return left_repeated == text;
        });
    }
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    let (acc_part_1, acc_part_2): (Vec<_>, Vec<_>) = text_input
        .lines()
        .collect::<Vec<_>>()
        .join("")
        .split(',')
        .par_bridge()
        .into_par_iter()
        .fold(
            || (0, 0),
            |mut acc, product_id_text| {
                debug!("Product ID text: {product_id_text}");

                let Some((first, last)) = product_id_text.split_once('-') else {
                    warn!("Input didn't have '-'");
                    return acc;
                };

                let Ok(first_id) = ProductID::from_string(first).inspect_err(warn_handler) else {
                    return acc;
                };

                let Ok(last_id) = ProductID::from_string(last).inspect_err(warn_handler) else {
                    return acc;
                };

                let product_id_range = ProductIDRange {
                    first: first_id,
                    last: last_id,
                };

                let (acc_part_1, acc_part_2): (Vec<_>, Vec<_>) = product_id_range
                    .into_par_iter()
                    .fold(
                        || (0, 0),
                        |mut acc, product_id: ProductID| {
                            if !product_id.is_valid_part_1() {
                                acc.0 += *product_id;
                            }

                            if !product_id.is_valid_part_2() {
                                acc.1 += *product_id;
                            }

                            return acc;
                        },
                    )
                    .unzip();

                acc.0 += acc_part_1.into_par_iter().sum::<u64>();
                acc.1 += acc_part_2.into_par_iter().sum::<u64>();

                return acc;
            },
        )
        .unzip();

    solution.part_1 = acc_part_1.into_par_iter().sum();
    solution.part_2 = acc_part_2.into_par_iter().sum();

    info!("{solution}");
    return solution;
}
