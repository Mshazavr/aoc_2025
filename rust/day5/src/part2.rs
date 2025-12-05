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
    ranges: Vec<Range>
}

impl Puzzle {
    fn from(text: String) -> Self {
        let mut ranges: Vec<Range> = vec![];
        for line in text.lines() {
            if line == "" {
                break
            }

            ranges.push(Range::from(&line));
        }

        Self {
            ranges
        }
    }

    fn get_total_range_coverage(&self) -> usize {
        let mut pairs: Vec<(i64, i64)>= self.ranges.iter().map(
            |range| (range.left, range.right)
        ).collect();

        pairs.sort();

        let mut last_right: i64 = -1;
        let mut total_coverage = 0;
        for (left, right) in pairs {
            if last_right < left {
                total_coverage += right - left + 1;
                last_right = right;
            }
            else if last_right < right {
                total_coverage += right - last_right;
                last_right = right;
            }
        }

        total_coverage as usize
    }
}

fn parse_input(run_config: &RunConfig) -> Puzzle {
    Puzzle::from(read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> usize {
    let puzzle = parse_input(run_config);
    puzzle.get_total_range_coverage()
}