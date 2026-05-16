use aoc2025::prelude::*;
use dashmap::DashMap;
use derive_more::Display;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

const INPUT_TEST: &str = include_str!("2025_06_input_test.txt");
const INPUT: &str = include_str!("2025_06_input.txt");

fn main() {
    init_tracing();

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        debug_assert_eq!(part_1, 4277556);
        debug_assert_eq!(part_2, 3263827);
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

    let mut lines_iter = text_input.lines();
    let operator_line = lines_iter
        .next_back()
        .expect("Input should have at least 1 line");

    let mut calculator = Calculator::from_line(operator_line);
    debug!("calculator: {calculator:?}");

    solution.part_1 = calculator.calculate_sum_part_1(lines_iter.clone().par_bridge()) as _;
    debug!("");
    solution.part_2 = calculator.calculate_sum_part_2(lines_iter) as _;
    debug!("");

    info!("{solution}");
    return solution;
}

#[derive(Clone, Debug)]
pub struct Calculator(pub DashMap<usize, (usize, Operation, Vec<usize>)>);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Operation {
    Add,
    Mul,
}

impl Calculator {
    pub fn from_line(line: &str) -> Self {
        let calculator = DashMap::new();

        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(token_i, op_str)| {
                let operation = Operation::try_from_str(op_str).expect("Invalid operation");

                let pos_x = line
                    .char_indices()
                    .filter(|(_, c)| *c == '+' || *c == '*')
                    .nth(token_i)
                    .map(|(i, _)| i)
                    .expect("Operator position");

                calculator.insert(token_i, (pos_x, operation, vec![]));
            });

        Self(calculator)
    }

    pub fn calculate_sum_part_1<'a>(
        &mut self,
        lines_iter: impl ParallelIterator<Item = &'a str>,
    ) -> usize {
        lines_iter.for_each(|line| {
            line.split_ascii_whitespace()
                .enumerate()
                .par_bridge()
                .for_each(|(ops_i, value)| {
                    debug!("i: {ops_i}; value: {value}");

                    if let Ok(value_parsed) = value.parse::<usize>() {
                        self.0
                            .get_mut(&ops_i)
                            .expect("Should exist")
                            .2
                            .push(value_parsed);
                    }
                })
        });

        return self.calculate_sum_and_clear();
    }

    pub fn calculate_sum_part_2<'a>(&mut self, lines_iter: impl Iterator<Item = &'a str>) -> usize {
        let rows = lines_iter.collect::<Vec<_>>();
        let max_x = rows.iter().map(|row| row.len()).max().unwrap_or(0);

        let mut operators = self
            .0
            .iter()
            .map(|entry| {
                let (operator_index, operation, _) = *entry.value();
                (operator_index, operation)
            })
            .collect::<Vec<_>>();

        operators.sort_unstable_by_key(|(operator_index, _)| *operator_index);

        operators
            .iter()
            .enumerate()
            .map(|(i, (operator_index, operation))| {
                let right_bound = operators
                    .get(i + 1)
                    .map(|(next_x, _)| next_x.saturating_sub(2))
                    .unwrap_or_else(|| max_x.saturating_sub(1));

                let mut values = Vec::<usize>::new();

                for x in (*operator_index..=right_bound).rev() {
                    let mut digits = String::new();

                    for row in &rows {
                        if let Some(&byte) = row.as_bytes().get(x)
                            && byte.is_ascii_digit()
                        {
                            digits.push(byte as char);
                        }
                    }

                    if !digits.is_empty() {
                        values.push(digits.parse::<usize>().expect("valid number"));
                    }
                }

                operation.batch_calculate(values)
            })
            .sum()
    }

    pub fn calculate_sum_and_clear(&mut self) -> usize {
        self.0
            .par_iter_mut()
            .map(|mut guard| {
                let (_, operation, batched) = guard.value_mut();
                let result = operation.batch_calculate(batched.clone());

                batched.clear();

                return result;
            })
            .sum()
    }
}

impl Operation {
    pub fn try_from_str(str: &str) -> Result<Self> {
        match str {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err("Unknown char".into()),
        }
    }

    pub fn batch_calculate(&self, batched: impl IntoParallelIterator<Item = usize>) -> usize {
        match self {
            Operation::Add => batched.into_par_iter().sum(),
            Operation::Mul => batched.into_par_iter().reduce(|| 1, |a, b| a * b),
        }
    }
}
