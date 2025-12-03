#![feature(int_roundings)]

use std::fmt::Display;

use aoc2025::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

const INPUT_TEST: &str = include_str!("2025_02_input_test.txt");
const INPUT_1: &str = include_str!("2025_02_input_1.txt");
const INPUT_2: &str = include_str!("2025_02_input_2.txt");

fn main() {
    init_tracing_debug();

    {
        assert!(!ProductID::from_string("55").unwrap().is_valid_part_1());
        assert!(!ProductID::from_string("6464").unwrap().is_valid_part_1());
        assert!(!ProductID::from_string("123123").unwrap().is_valid_part_1());
        assert!(ProductID::from_string("0101").is_err());
    }

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, .. } = solution;
        assert_eq!(part_1, 1227775554);
    }

    solution(INPUT_1);
    // solution(INPUT_2);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    /// Sum of invalid ProductIDs (digit repeated twice)
    pub part_1: u64,
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
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    solution.part_1 = text_input
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(',')
        .par_bridge()
        .into_par_iter()
        .fold(
            || 0,
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

                acc += product_id_range
                    .into_par_iter()
                    .fold(
                        || 0,
                        |mut acc, product_id: ProductID| {
                            if !product_id.is_valid_part_1() {
                                acc += *product_id;
                            }

                            return acc;
                        },
                    )
                    .sum::<u64>();

                return acc;
            },
        )
        .sum();

    info!("{solution}");
    return solution;
}
