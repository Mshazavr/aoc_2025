use std::collections::VecDeque;
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

    fn get_num_tachyon_splits(&self) -> i64 {
        let mut splitter_queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visit_map: Vec<Vec<bool>> = vec![vec![false; self.map[0].len()]; self.map.len()];
        let mut num_tachyon_splits = 0;

        let (mut x, y) = self.start_coords;
        x += 1;
        while x < self.map.len() && self.map[x][y] == Cell::Empty {
            x += 1;
        }

        if x == self.map.len() || !(self.map[x][y] == Cell::Splitter) {
            return 0;
        }

        visit_map[x][y] = true;
        splitter_queue.push_back((x, y));
        while !splitter_queue.is_empty() {
            let (x, y) = splitter_queue.pop_front().unwrap();
            num_tachyon_splits += 1;

            let column_indices: [isize; 2] = [(y as isize) - 1, (y as isize) + 1];
            for cur_y in column_indices {
                if cur_y < 0 || cur_y >= self.map[0].len() as isize {
                    continue;
                }

                let mut cur_x = x;
                while cur_x < self.map.len() && self.map[cur_x][cur_y as usize] == Cell::Empty {
                    cur_x += 1;
                }
                if cur_x < self.map.len() && self.map[cur_x][cur_y as usize] == Cell::Splitter {
                    if !visit_map[cur_x][cur_y as usize] {
                        visit_map[cur_x][cur_y as usize] = true;
                        splitter_queue.push_back((cur_x, cur_y as usize));
                    }
                }
            }
        }

        num_tachyon_splits
    }
}

fn parse_input(run_config: &RunConfig) -> Manifold {
    Manifold::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> i64 {
    let manifold = parse_input(run_config);
    manifold.get_num_tachyon_splits()
}
