use aoc2025::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use parking_lot::Mutex;
use rayon::{
    iter::{IntoParallelRefMutIterator, ParallelBridge, ParallelIterator},
    str::ParallelString,
};
use std::{collections::HashMap, sync::Arc};

const INPUT_TEST: &str = include_str!("2025_04_input_test.txt");
const INPUT: &str = include_str!("2025_04_input.txt");

fn main() {
    init_tracing();

    {
        let grids = Grid::from_lines(INPUT_TEST);
        assert_eq!(grids.count_adjacent((0, 0)), 2);
        assert_eq!(grids.count_adjacent((1, 1)), 6);
        assert_eq!(grids.count_adjacent((4, 4)), 8);
    }

    {
        let solution = solution(INPUT_TEST);
        let Solution { part_1, part_2 } = solution;
        debug_assert_eq!(part_1, 13);
        debug_assert_eq!(part_2, 43);
    }

    solution(INPUT);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    // Sum of (fewer than four adjacent) accessible papers rolls
    pub part_1: u64,
    // Sum of (fewer than four adjacent) accessible papers rolls recursively until no more
    pub part_2: u64,
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    let mut grids = Grid::from_lines(text_input);

    solution.part_1 = grids.count_all_fewer_than_4();
    solution.part_2 += solution.part_1;

    loop {
        let count = grids.count_all_fewer_than_4();

        if count == 0 {
            break;
        }

        solution.part_2 += count
    }

    info!("{solution}");
    return solution;
}

#[derive(Clone, PartialEq, Eq, Debug, Deref, DerefMut, Display)]
#[display("{_0:?}")]
pub struct Grid(pub HashMap<(usize, usize), StuffKind>);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
pub enum StuffKind {
    #[display(".")]
    Empty,
    #[display("@")]
    PaperRoll,
}

impl Grid {
    pub fn from_lines(lines: &str) -> Self {
        let map = Arc::new(Mutex::new(HashMap::new()));

        lines
            .lines()
            .enumerate()
            .par_bridge()
            .for_each(|(pos_y, line)| {
                line.par_char_indices().for_each(|(pos_x, c)| {
                    if map
                        .lock()
                        .insert((pos_x, pos_y), StuffKind::from_char(c).unwrap())
                        .is_some()
                    {
                        unreachable!("Positions should not be duplicated")
                    };
                });
            });

        return Self((*(*map).lock()).clone());
    }

    pub fn count_adjacent(&self, (pos_x, pos_y): (usize, usize)) -> u8 {
        let pos_x_sub_1 = pos_x.checked_sub(1);
        let pos_y_sub_1 = pos_y.checked_sub(1);

        let mut counter = 0;

        if let Some(top_left) = pos_y_sub_1
            .and_then(|y| pos_x_sub_1.map(|x| self.get(&(x, y))))
            .flatten()
            && *top_left == StuffKind::PaperRoll
        {
            debug!("top_left");
            counter += 1;
        };

        if let Some(top_center) = pos_y_sub_1.and_then(|y| self.get(&(pos_x, y)))
            && *top_center == StuffKind::PaperRoll
        {
            debug!("top_center");
            counter += 1;
        };

        if let Some(top_right) = pos_y_sub_1.and_then(|y| self.get(&(pos_x + 1, y)))
            && *top_right == StuffKind::PaperRoll
        {
            debug!("top_right");
            counter += 1;
        };

        if let Some(middle_left) = pos_x_sub_1.and_then(|x| self.get(&(x, pos_y)))
            && *middle_left == StuffKind::PaperRoll
        {
            debug!("middle_left");
            counter += 1;
        };
        if let Some(middle_right) = self.get(&(pos_x + 1, pos_y))
            && *middle_right == StuffKind::PaperRoll
        {
            debug!("middle_right");
            counter += 1;
        };
        if let Some(bottom_left) = pos_x_sub_1.and_then(|x| self.get(&(x, (pos_y + 1))))
            && *bottom_left == StuffKind::PaperRoll
        {
            debug!("bottom_left");
            counter += 1;
        };
        if let Some(bottom_center) = self.get(&(pos_x, (pos_y + 1)))
            && *bottom_center == StuffKind::PaperRoll
        {
            debug!("bottom_center");
            counter += 1;
        };
        if let Some(bottom_right) = self.get(&((pos_x + 1), (pos_y + 1)))
            && *bottom_right == StuffKind::PaperRoll
        {
            debug!("bottom_right");
            counter += 1;
        };

        debug!("pos: {pos_x}, {pos_y}; counter: {counter}");

        counter
    }

    pub fn count_all_fewer_than_4(&mut self) -> u64 {
        let cloned_grid = self.clone();

        self.par_iter_mut()
            .filter(|(pos, kind)| {
                if **kind != StuffKind::PaperRoll {
                    return false;
                };

                let count = cloned_grid.count_adjacent(**pos);

                if count >= 4 {
                    return false;
                }

                debug!("take");
                return true;
            })
            .map(|(_, kind)| {
                *kind = StuffKind::Empty;
            })
            .count() as _
    }
}

impl StuffKind {
    pub const fn from_char(char: char) -> Result<Self> {
        Ok(match char {
            '.' => Self::Empty,
            '@' => Self::PaperRoll,
            _ => unreachable!(),
        })
    }
}
