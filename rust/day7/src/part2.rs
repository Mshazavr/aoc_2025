use std::fs::read_to_string;

use crate::RunConfig;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

impl Cell {
    fn from(text: char) -> Self {
        match text {
            '.' => Cell::Empty,
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            _ => panic!(""),
        }
    }
}

#[derive(Debug)]
struct Manifold {
    map: Vec<Vec<Cell>>,
    start_coords: (usize, usize),
}

impl Manifold {
    fn from(text: &str) -> Self {
        let map: Vec<Vec<Cell>> = text
            .lines()
            .map(|line| line.chars().map(|c| Cell::from(c)).collect())
            .collect();
        let mut start_coords: Option<(usize, usize)> = None;

        for (idx, cell) in map[0].iter().enumerate() {
            if let Cell::Start = cell {
                start_coords = Some((0, idx));
                break;
            }
        }
        Self {
            map,
            start_coords: start_coords.unwrap(),
        }
    }

    fn get_num_tachyon_paths(&self) -> i64 {
        let mut num_paths_to: Vec<Vec<i64>> = vec![vec![0; self.map[0].len()]; self.map.len()];

        num_paths_to[self.start_coords.0][self.start_coords.1] = 1;
        for row_idx in 1..self.map.len() {
            for col_idx in 0..self.map[0].len() {
                if self.map[row_idx - 1][col_idx] != Cell::Splitter {
                    num_paths_to[row_idx][col_idx] += num_paths_to[row_idx - 1][col_idx];
                }
                if col_idx > 0 && self.map[row_idx][col_idx - 1] == Cell::Splitter {
                    num_paths_to[row_idx][col_idx] += num_paths_to[row_idx - 1][col_idx - 1];
                }
                if col_idx + 1 < self.map[0].len()
                    && self.map[row_idx][col_idx + 1] == Cell::Splitter
                {
                    num_paths_to[row_idx][col_idx] += num_paths_to[row_idx - 1][col_idx + 1];
                }
            }
        }

        num_paths_to[self.map.len() - 1].iter().sum()
    }
}

fn parse_input(run_config: &RunConfig) -> Manifold {
    Manifold::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> i64 {
    let manifold = parse_input(run_config);
    manifold.get_num_tachyon_paths()
}
