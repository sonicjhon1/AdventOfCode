#![feature(exact_length_collection)]
#![feature(array_windows)]

use aoc2025::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    str::ParallelString,
};

const INPUT_TEST: &str = include_str!("2025_03_input_test.txt");
const INPUT: &str = include_str!("2025_03_input.txt");

fn main() {
    init_tracing();

    {
        debug_assert_eq!(
            BatteryJoltage::<2>::from_bank("987654321111111").as_number(),
            98
        );
        debug_assert_eq!(
            BatteryJoltage::<2>::from_bank("811111111111119").as_number(),
            89
        );
        debug_assert_eq!(
            BatteryJoltage::<2>::from_bank("234234234234278").as_number(),
            78
        );
        debug_assert_eq!(
            BatteryJoltage::<2>::from_bank("818181911112111").as_number(),
            92
        );

        debug_assert_eq!(
            BatteryJoltage::<12>::from_bank("987654321111111").as_number(),
            987654321111
        );
        debug_assert_eq!(
            BatteryJoltage::<12>::from_bank("811111111111119").as_number(),
            811111111119
        );
        debug_assert_eq!(
            BatteryJoltage::<12>::from_bank("234234234234278").as_number(),
            434234234278
        );
        debug_assert_eq!(
            BatteryJoltage::<12>::from_bank("818181911112111").as_number(),
            888911112111
        );
    }

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        debug_assert_eq!(part_1, 357);
        debug_assert_eq!(part_2, 3121910778619);
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

    let (part_1s, part_2s): (Vec<_>, Vec<_>) = text_input
        .par_lines()
        .fold(
            || (0, 0),
            |(mut part_1, mut part_2), line| {
                part_1 += BatteryJoltage::<2>::from_bank(line).as_number();
                part_2 += BatteryJoltage::<12>::from_bank(line).as_number();

                return (part_1, part_2);
            },
        )
        .unzip();

    solution.part_1 = part_1s.par_iter().sum::<u64>();
    solution.part_2 = part_2s.par_iter().sum::<u64>();

    info!("{solution}");
    return solution;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deref, DerefMut, Display)]
#[display("{}", self.as_number())]
pub struct BatteryJoltage<const N: usize>(pub [u32; N]);

impl<const N: usize> BatteryJoltage<N> {
    pub fn as_number(&self) -> u64 {
        self.0
            .into_par_iter()
            .rev()
            .enumerate()
            .map(|(i, val)| {
                let multiplier = 10u64.pow(i as _);

                return multiplier * (val as u64);
            })
            .sum()
    }

    pub fn from_bank(bank: &str) -> Self {
        debug!("Battery bank: {bank}");

        let mut bank_iter = bank.chars().map(|c| c.to_digit(10).unwrap());
        let starting_joltage =
            BatteryJoltage(bank_iter.by_ref().take(N).collect_array::<N>().unwrap());

        bank_iter.fold(starting_joltage, |mut acc_joltage, value| {
            let smaller_i = acc_joltage
                .array_windows::<2>()
                .into_iter()
                .enumerate()
                .find_map(|(i, [a, b])| (a < b).then_some(i));

            debug!("acc_joltage: {acc_joltage}; value: {value}");

            if let Some(smaller_i) = smaller_i {
                acc_joltage.copy_within((smaller_i + 1)..N, smaller_i);
                acc_joltage[N - 1] = value;

                debug!("smaller_i {smaller_i}");
            } else if acc_joltage[N - 1] < value {
                acc_joltage[N - 1] = value;
            };

            debug!("acc_joltage: {acc_joltage}");

            debug!("");
            return acc_joltage;
        })
    }
}
