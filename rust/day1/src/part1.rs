use std::fs::read_to_string;
use std::slice;

use crate::RunConfig;

#[derive(Debug)]
struct Position(i16);

#[derive(Debug)]
enum Operation {
    Left(i16),
    Right(i16),
}

#[derive(Debug)]
struct PasswordSequence(Vec<Operation>);

impl PasswordSequence {
    fn iter(&self) -> slice::Iter<'_, Operation> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a PasswordSequence {
    type Item = &'a Operation;
    type IntoIter = slice::Iter<'a, Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn move_position(position: &mut Position, operation: &Operation) {
    let change: i16 = match operation {
        Operation::Left(n) => -*n,
        Operation::Right(n) => *n,
    };
    position.0 = (position.0 + change + 100) % 100;
}

fn apply_sequence_and_count_zeroes(sequence: &PasswordSequence) -> u32 {
    let mut position = Position(50);
    let mut num_zeroes: u32 = 0;
    for operation in sequence {
        move_position(&mut position, operation);
        if position.0 == 0 {
            num_zeroes += 1;

            #[cfg(debug_assertions)]
            println!("Moved to {position:?}. HIT!");
        }

        #[cfg(debug_assertions)]
        println!("Moved to {position:?}.");
    }
    num_zeroes
}

fn parse_input(run_config: &RunConfig) -> PasswordSequence {
    PasswordSequence(
        read_to_string(run_config.get_test_path())
            .unwrap()
            .lines()
            .map(|line| {
                let direction: char = line.chars().nth(0).unwrap();
                let magnitude: i16 = line[1..].parse::<i16>().unwrap();
                match direction {
                    'L' => Operation::Left(magnitude),
                    'R' => Operation::Right(magnitude),
                    _ => panic!("Expected the first letter to be L or R."),
                }
            })
            .collect(),
    )
}

pub fn run(run_config: &RunConfig) -> u32 {
    let input = parse_input(run_config);
    let result = apply_sequence_and_count_zeroes(&input);
    result
}
