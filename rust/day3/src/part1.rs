use std::fs::read_to_string;

use crate::RunConfig;

struct Bank(Vec<u32>);

impl Bank {
    fn get_max_power(&self) -> Option<u32> {
        if self.0.len() <= 1 {
            return None;
        }

        let mut max_val_and_idx: (u32, usize) = (self.0[0], 0);
        let mut second_max_val_behind: Option<u32> = None;
        for (idx, battery) in self.0.iter().enumerate() {
            if *battery > max_val_and_idx.0 {
                second_max_val_behind = Some(max_val_and_idx.0);
                max_val_and_idx = (*battery, idx);
            }
            if max_val_and_idx.0 == 9 {
                break;
            }
        }

        if max_val_and_idx.1 + 1 == self.0.len() {
            return Some(max_val_and_idx.0 + 10 * second_max_val_behind.unwrap());
        }

        let second_max_val_after: u32 = *self.0[(max_val_and_idx.1 + 1)..].iter().max().unwrap();

        Some(second_max_val_after + 10 * max_val_and_idx.0)
    }
}

fn get_max_total_power(banks: &Vec<Bank>) -> u32 {
    banks.iter().map(|bank| bank.get_max_power().unwrap()).sum()
}

fn parse_input(run_config: &RunConfig) -> Vec<Bank> {
    read_to_string(run_config.get_test_path())
        .unwrap()
        .lines()
        .map(|line| Bank(line.chars().map(|c| c.to_digit(10).unwrap()).collect()))
        .collect()
}

pub fn run(run_config: &RunConfig) -> u32 {
    let input = parse_input(run_config);
    let result = get_max_total_power(&input);
    result
}
