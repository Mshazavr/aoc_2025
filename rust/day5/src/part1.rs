use std::fs::read_to_string;

use crate::RunConfig;


struct Range {
    left: i64,
    right: i64,
}

impl Range {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split('-').collect();
        Range {
            left: parts[0].parse::<i64>().unwrap(),
            right: parts[1].parse::<i64>().unwrap()
        }
    }
}


struct Puzzle {
    ranges: Vec<Range>,
    ids: Vec<i64>
}

impl Puzzle {
    fn from(text: String) -> Self {
        let mut ranges: Vec<Range> = vec![];
        let mut ids: Vec<i64> = vec![];

        let mut mode: i64 = 0;
        for line in text.lines() {
            if line == "" {
                mode ^= 1;
                continue;
            }

            if mode == 0 {
                ranges.push(Range::from(&line));
            }
            else {
                ids.push(line.parse::<i64>().unwrap());
            }
        }

        Self {
            ranges,
            ids
        }
    }

    fn get_num_valid_ids(&self) -> usize {
        self.ids.iter().filter(|id| {
            self.ranges
            .iter()
            .any(|range| **id >= range.left && **id <= range.right)
        }).count()
    }
}

fn parse_input(run_config: &RunConfig) -> Puzzle {
    Puzzle::from(read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> usize {
    let puzzle = parse_input(run_config);
    puzzle.get_num_valid_ids()
}