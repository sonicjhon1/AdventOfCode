#![feature(int_roundings)]

use aoc2025::prelude::*;
use derive_more::Display;

const INPUT_TEST: &str = include_str!("2025_01_input_test.txt");
const INPUT_1: &str = include_str!("2025_01_input_1.txt");
const INPUT_2: &str = include_str!("2025_01_input_1.txt");

fn main() {
    init_tracing();

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        assert_eq!(part_1, 3);
        assert_eq!(part_2, 6);
    }

    solution(INPUT_1);
    solution(INPUT_2);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    /// Dial ended perfectly at 0
    pub part_1: u64,
    /// Dial ended perfectly at 0 OR clicked pass 0
    pub part_2: u64,
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut dial = WrapNumber {
        value: 50,
        n_wrap: 0,
    };

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    for line in text_input.lines() {
        debug!("Line: {line}");

        let (prefix, value) = line.split_at(1);

        let delta = match (prefix, value.parse::<i32>()) {
            ("L", Ok(value_parsed)) => -value_parsed,
            ("R", Ok(value_parsed)) => value_parsed,
            _ => unreachable!(),
        };

        wrapped_add(&mut dial, delta);

        debug!("Dial: {}", dial.value);

        if dial.value == 0 {
            solution.part_1 += 1;
        }

        debug!("");
    }

    solution.part_2 = dial.n_wrap;

    info!("{solution}");
    return solution;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct WrapNumber {
    pub value: i32,
    pub n_wrap: u64,
}

const WRAP_AT: i32 = 100;

fn wrapped_add(number: &mut WrapNumber, rhs: i32) {
    let total_number = number.value + rhs;
    let revolution = (total_number.abs() / WRAP_AT) as u64;
    let new_number = total_number.rem_euclid(WRAP_AT);

    if revolution > 0 {
        number.n_wrap += revolution;
        debug!("n_wrap: Revolution {}", number.n_wrap);
    }

    if number.value != 0 && total_number <= 0 {
        number.n_wrap += 1;
        debug!("n_wrap: Sign {}", number.n_wrap);
    }

    number.value = new_number;
}
