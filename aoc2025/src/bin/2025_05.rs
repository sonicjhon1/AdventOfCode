#![feature(get_disjoint_mut_helpers)]

use aoc2025::prelude::*;
use derive_more::{Display, core::slice::GetDisjointMutIndex};
use rayon::{
    iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    str::ParallelString,
};
use std::ops::RangeInclusive;

const INPUT_TEST: &str = include_str!("2025_05_input_test.txt");
const INPUT: &str = include_str!("2025_05_input.txt");

fn main() {
    init_tracing();

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        debug_assert_eq!(part_1, 3);
        debug_assert_eq!(part_2, 14);
    }

    solution(INPUT);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    pub part_1: u64,
    pub part_2: u64,
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    let database = Database::from_lines(text_input);

    solution.part_1 = database.count_fresh_ingredients() as _;
    solution.part_2 = database.fresh_range_flatten() as _;

    info!("{solution}");
    return solution;
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Database {
    freshs: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Database {
    pub fn from_lines(lines: &str) -> Self {
        let (text_freshs, text_ingredients) = lines
            .split_once("\n\n")
            .expect("Input should contain an empty line");

        let ingredients_fresh = text_freshs
            .par_lines()
            .map(|line| {
                let (start, end) = line
                    .split_once('-')
                    .expect("Fresh range should contain '-'");

                RangeInclusive::new(
                    start.parse().expect("Fresh start should be a number"),
                    end.parse().expect("Fresh last should be a number"),
                )
            })
            .collect();

        let ingredients = text_ingredients
            .par_lines()
            .map(|line| {
                line.parse::<usize>()
                    .expect("ingredient should be a number")
            })
            .collect();

        return Self {
            freshs: ingredients_fresh,
            ingredients,
        };
    }

    pub fn count_fresh_ingredients(&self) -> usize {
        return self
            .ingredients
            .par_iter()
            .filter(|ingredient| {
                self.freshs
                    .par_iter()
                    .clone()
                    .any(|ingredients_fresh| ingredients_fresh.contains(ingredient))
            })
            .count();
    }

    pub fn fresh_range_flatten(self) -> usize {
        let mut sorted = self.freshs;
        sorted.sort_by(|a, b| a.start().cmp(b.start()));

        let mut set = vec![];

        sorted.into_iter().for_each(|freshes| {
            if let Some(overlap_fresh) = set.par_iter_mut().find_any(|f| freshes.is_overlapping(f)) {
                let start = freshes.start().min(overlap_fresh.start());
                let end = freshes.end().max(overlap_fresh.end());

                *overlap_fresh = RangeInclusive::new(*start, *end);
            } else {
                set.push(freshes);
            };

            debug!("set: {set:?}");
        });

        return set
            .par_iter()
            .map(|freshes| (freshes.end() + 1) - freshes.start())
            .sum();
    }
}
