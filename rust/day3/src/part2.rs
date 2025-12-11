use std::fs::read_to_string;

use crate::RunConfig;

struct Bank(Vec<u64>);

impl Bank {
    fn get_max_power(&self) -> Option<u64> {
        if self.0.len() <= 12 {
            return None;
        }

        let mut base: u64 = 10_u64.pow(11);
        let mut cur_start: usize = 0;
        let mut total_power: u64 = 0;
        for iteration in 0..12 {
            let mut max: u64 = self.0[cur_start];
            let mut argmax: usize = cur_start;
            for idx in cur_start..(self.0.len() - 11 + iteration) {
                if max < self.0[idx] {
                    max = self.0[idx];
                    argmax = idx;
                }
            }

            total_power += base * max;
            base /= 10;
            cur_start = argmax + 1;
        }

        Some(total_power)
    }
}

fn get_max_total_power(banks: &Vec<Bank>) -> u64 {
    banks.iter().map(|bank| bank.get_max_power().unwrap()).sum()
}

fn parse_input(run_config: &RunConfig) -> Vec<Bank> {
    read_to_string(run_config.get_test_path())
        .unwrap()
        .lines()
        .map(|line| {
            Bank(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect(),
            )
        })
        .collect()
}

pub fn run(run_config: &RunConfig) -> u64 {
    let input = parse_input(run_config);
    let result = get_max_total_power(&input);
    result
}
