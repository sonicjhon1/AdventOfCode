use aoc2025::prelude::*;
use derive_more::{Deref, DerefMut, Display};
use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display, sync::atomic::AtomicUsize};

const INPUT_TEST: &str = include_str!("2025_07_input_test.txt");
const INPUT: &str = include_str!("2025_07_input.txt");

fn main() {
    init_tracing_debug();

    let Solution { part_1, part_2 } = solution(INPUT_TEST);
    debug_assert_eq!(part_1, 21);
    debug_assert_eq!(part_2, 40);

    solution(INPUT);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[display("Solution: [part_1: ({part_1}), part_2: ({part_2})]")]
pub struct Solution {
    pub part_1: usize,
    pub part_2: usize,
}

fn solution(text_input: &str) -> Solution {
    debug!("\n---Input---\n{text_input}\n---EOF---");

    let mut solution = Solution {
        part_1: 0,
        part_2: 0,
    };

    let grid_map = GridMap::from_lines(text_input);
    let mut grid = Grid {
        classic_map: grid_map.clone(),
        quantum_maps: vec![grid_map],
    };

    let mut beam_split_counter = 0;
    loop {
        let grid_classic_map_before = grid.classic_map.clone();
        grid.map_simulate_part_1(&mut beam_split_counter);

        debug!("\n---Simulated map---\n{}", grid.classic_map);

        if grid_classic_map_before == grid.classic_map {
            debug!("Map is fully simulated.");
            break;
        }
    }

    // let finished_maps_counter = AtomicUsize::new(0);
    // loop {
    //     grid.map_simulate_part_2(&finished_maps_counter);

    //     for (map_index, map) in grid.quantum_maps.iter().enumerate() {
    //         debug!("\n---Simulated map ({map_index})---\n{map}");
    //     }

    //     debug!(
    //         "Maps finished: ({}/{})",
    //         finished_maps_counter.load(std::sync::atomic::Ordering::Relaxed),
    //         grid.quantum_maps.len()
    //     );

    //     if grid.quantum_maps.is_empty() {
    //         debug!("Maps are fully simulated.");
    //         break;
    //     }
    // }

    solution.part_1 = beam_split_counter;
    // solution.part_2 = finished_maps_counter.load(std::sync::atomic::Ordering::Relaxed);
    solution.part_2 = grid.solve_part_2();

    info!("{solution}");
    return solution;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid {
    classic_map: GridMap,
    quantum_maps: Vec<GridMap>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deref, DerefMut)]
/// GridMap[y_pos][x_pos]
pub struct GridMap(Vec<Vec<GridToken>>);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
pub enum GridToken {
    #[display("S")]
    BeamEnter,
    #[display("|")]
    Beam,
    #[display(".")]
    Empty,
    #[display("^")]
    Splitter,
    #[display("<")]
    SplitterLeft,
    #[display(">")]
    SplitterRight,
}

impl Grid {
    pub fn map_simulate_part_1(&mut self, beam_split_counter: &mut usize) {
        self.classic_map.simulate_classic(beam_split_counter);
    }

    pub fn map_simulate_part_2(&mut self, finished_maps_counter: &AtomicUsize) {
        self.quantum_maps = self
            .quantum_maps
            .par_iter()
            .flat_map(|quantum_map| quantum_map.simulate_quantum(finished_maps_counter))
            .collect();
    }

    pub fn solve_part_2(&mut self) -> usize {
        self.quantum_maps
            .iter()
            .map(|map| {
                let rows = map.len();
                let cols = map.first().expect("Map should contain atleast 1 row").len();

                let start_col = map[0]
                    .iter()
                    .position(|token| *token == GridToken::BeamEnter)
                    .expect("First row should contain a (BeamEnter)");

                let mut cache = HashMap::new();

                fn dfs(
                    grid: &GridMap,
                    rows: usize,
                    cols: usize,
                    row: usize,
                    col: usize,
                    cache: &mut HashMap<(usize, usize), usize>,
                ) -> usize {
                    if row >= rows || col >= cols {
                        return 1;
                    }

                    if let Some(v) = cache.get(&(row, col)) {
                        return *v;
                    }

                    let token = grid[row][col];

                    let res = match token {
                        GridToken::Splitter => {
                            dfs(grid, rows, cols, row + 1, col - 1, cache)
                                + dfs(grid, rows, cols, row + 1, col + 1, cache)
                        }

                        GridToken::Empty | GridToken::Beam | GridToken::BeamEnter => {
                            dfs(grid, rows, cols, row + 1, col, cache)
                        }

                        _ => 0,
                    };

                    cache.insert((row, col), res);
                    res
                }

                dfs(map, rows, cols, 1, start_col, &mut cache)
            })
            .sum()
    }
}

impl GridMap {
    pub fn from_lines(lines: &str) -> Self {
        let map_tokens = lines
            .par_lines()
            .map(|line| {
                line.par_chars()
                    .map(|c| {
                        GridToken::try_from_str(&c.to_string())
                            .expect("Grid should only have valid token")
                    })
                    .collect()
            })
            .collect();

        let mut map = Self(map_tokens);
        map.propagate_beam_to_empties();

        return map;
    }

    pub fn simulate_classic(&mut self, beam_split_counter: &mut usize) {
        let mut new_map = self.clone();

        for ((tokens_y, tokens), (tokens_bottom_y, tokens_bottom)) in
            self.iter().enumerate().tuple_windows()
        {
            for (token_x, token) in tokens.iter().enumerate() {
                let token_bottom = tokens_bottom
                    .get(token_x)
                    .expect("Grid should have the same length");

                trace!(
                    "Grid: ({token_x:02}, {tokens_y:02}); token: ({token}); token_bottom: ({token_bottom})"
                );

                match token {
                    GridToken::BeamEnter | GridToken::Beam => match token_bottom {
                        GridToken::BeamEnter => unreachable!(),
                        GridToken::Beam => {}
                        GridToken::Empty => {
                            trace!(
                                "Replacing (Empty) with (Beam) at ({token_x:02}, {tokens_y:02})"
                            );

                            new_map[tokens_bottom_y][token_x] = GridToken::Beam;
                        }
                        GridToken::Splitter => {
                            let mut is_beam_splitted = false;

                            if let Some(token_bottom_left_x) = token_x.checked_sub(1) {
                                let token_bottom_left = new_map
                                    .get_mut(tokens_bottom_y)
                                    .unwrap()
                                    .get_mut(token_bottom_left_x)
                                    .unwrap();

                                match token_bottom_left {
                                    GridToken::Empty => {
                                        trace!(
                                            "Replacing (Empty) with (Beam) at ({token_bottom_left_x:02}, {tokens_bottom_y:02})"
                                        );

                                        *token_bottom_left = GridToken::Beam;
                                        is_beam_splitted = true;
                                    }
                                    GridToken::BeamEnter
                                    | GridToken::Beam
                                    | GridToken::Splitter
                                    | GridToken::SplitterLeft
                                    | GridToken::SplitterRight => {}
                                }
                            }

                            if let Some(token_bottom_right_x) = token_x.checked_add(1) {
                                let token_bottom_right = new_map
                                    .get_mut(tokens_bottom_y)
                                    .unwrap()
                                    .get_mut(token_bottom_right_x)
                                    .unwrap();

                                match token_bottom_right {
                                    GridToken::Empty => {
                                        trace!(
                                            "Replacing (Empty) with (Beam) at ({token_bottom_right_x:02}, {tokens_bottom_y:02})"
                                        );

                                        *token_bottom_right = GridToken::Beam;
                                        is_beam_splitted = true;
                                    }
                                    GridToken::BeamEnter
                                    | GridToken::Beam
                                    | GridToken::Splitter
                                    | GridToken::SplitterLeft
                                    | GridToken::SplitterRight => {}
                                }
                            };

                            if is_beam_splitted {
                                *beam_split_counter += 1;
                            }
                        }
                        GridToken::SplitterLeft | GridToken::SplitterRight => {}
                    },
                    GridToken::Empty
                    | GridToken::Splitter
                    | GridToken::SplitterLeft
                    | GridToken::SplitterRight => {}
                }
            }
        }

        *self = new_map;
    }

    fn simulate_quantum(&self, finished_maps_counter: &AtomicUsize) -> Vec<Self> {
        let mut new_quantum_maps = Vec::with_capacity(3);

        for ((tokens_bottom_y, tokens_bottom), (tokens_y, tokens)) in
            self.iter().enumerate().rev().tuple_windows()
        {
            for (token_x, token) in tokens.iter().enumerate() {
                let token_bottom = tokens_bottom
                    .get(token_x)
                    .expect("Grid should have the same length");

                trace!(
                    "Grid: ({token_x:02}, {tokens_y:02}); token: ({token}); token_bottom: ({token_bottom})"
                );

                match token {
                    GridToken::BeamEnter | GridToken::Beam => match token_bottom {
                        GridToken::BeamEnter | GridToken::Empty => unreachable!(),
                        GridToken::Splitter => {
                            if let Some(token_bottom_left_x) = token_x.checked_sub(1) {
                                let mut new_quantum_map = self.clone();
                                let token_bottom_left = new_quantum_map
                                    .get_mut(tokens_bottom_y)
                                    .unwrap()
                                    .get_mut(token_bottom_left_x)
                                    .unwrap();

                                match token_bottom_left {
                                    GridToken::Empty => {
                                        trace!(
                                            "Replacing (Empty) with (Beam) at ({token_bottom_left_x:02}, {tokens_bottom_y:02})"
                                        );

                                        *token_bottom_left = GridToken::Beam;
                                        new_quantum_map.propagate_beam_to_empties();
                                        new_quantum_map[tokens_bottom_y][token_x] =
                                            GridToken::SplitterLeft;
                                        new_quantum_maps.push(new_quantum_map);
                                    }
                                    GridToken::BeamEnter
                                    | GridToken::Beam
                                    | GridToken::Splitter
                                    | GridToken::SplitterLeft
                                    | GridToken::SplitterRight => {}
                                }
                            }

                            if let Some(token_bottom_right_x) = token_x.checked_add(1) {
                                let mut new_quantum_map = self.clone();
                                let token_bottom_right = new_quantum_map
                                    .get_mut(tokens_bottom_y)
                                    .unwrap()
                                    .get_mut(token_bottom_right_x)
                                    .unwrap();

                                match token_bottom_right {
                                    GridToken::Empty => {
                                        trace!(
                                            "Replacing (Empty) with (Beam) at ({token_bottom_right_x:02}, {tokens_bottom_y:02})"
                                        );

                                        *token_bottom_right = GridToken::Beam;
                                        new_quantum_map.propagate_beam_to_empties();
                                        new_quantum_map[tokens_bottom_y][token_x] =
                                            GridToken::SplitterRight;
                                        new_quantum_maps.push(new_quantum_map);
                                    }
                                    GridToken::BeamEnter
                                    | GridToken::Beam
                                    | GridToken::Splitter
                                    | GridToken::SplitterLeft
                                    | GridToken::SplitterRight => {}
                                }
                            };
                        }
                        GridToken::Beam | GridToken::SplitterLeft | GridToken::SplitterRight => {}
                    },
                    GridToken::Empty
                    | GridToken::Splitter
                    | GridToken::SplitterLeft
                    | GridToken::SplitterRight => {}
                }
            }
        }

        if new_quantum_maps.is_empty() {
            finished_maps_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        return new_quantum_maps;
    }

    fn propagate_beam_to_empties(&mut self) {
        let mut new_map = self.clone();

        loop {
            let new_map_before = new_map.clone();

            for ((tokens_y, tokens), (tokens_bottom_y, tokens_bottom)) in
                new_map.clone().iter().enumerate().tuple_windows()
            {
                for (token_x, token) in tokens.iter().enumerate() {
                    let token_bottom = tokens_bottom[token_x];

                    trace!(
                        "Grid: ({token_x:02}, {tokens_y:02}); token: ({token}); token_bottom: ({token_bottom})"
                    );

                    match token {
                        GridToken::BeamEnter | GridToken::Beam => match token_bottom {
                            GridToken::BeamEnter => unreachable!(),
                            GridToken::Empty => {
                                trace!(
                                    "Replacing (Empty) with (Beam) at ({token_x:02}, {tokens_y:02})"
                                );
                                new_map[tokens_bottom_y][token_x] = GridToken::Beam;
                            }
                            GridToken::Beam
                            | GridToken::Splitter
                            | GridToken::SplitterLeft
                            | GridToken::SplitterRight => {}
                        },
                        GridToken::Empty
                        | GridToken::Splitter
                        | GridToken::SplitterLeft
                        | GridToken::SplitterRight => {}
                    }
                }
            }

            if new_map == new_map_before {
                break;
            }
        }

        *self = new_map;
    }
}

impl Display for GridMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tokens in self.iter() {
            writeln!(f)?;

            for token in tokens.iter() {
                write!(f, "{token} ")?;
            }
        }

        Ok(())
    }
}

impl GridToken {
    pub fn try_from_str(str: &str) -> Result<Self> {
        match str {
            "S" => Ok(Self::BeamEnter),
            "|" => Ok(Self::Beam),
            "." => Ok(Self::Empty),
            "^" => Ok(Self::Splitter),
            "<" => Ok(Self::SplitterLeft),
            ">" => Ok(Self::SplitterRight),
            _ => Err(format!("Unknown char: ({str})").into()),
        }
    }
}
