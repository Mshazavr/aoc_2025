use std::slice;
use std::fs::read_to_string;

use crate::RunConfig;

#[derive(Debug)]
struct Position(i32);

#[derive(Debug)]
enum Operation {
    Left(i32),
    Right(i32)
}

#[derive(Debug)]
struct PasswordSequence (Vec<Operation>);

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

fn move_position(position: &mut Position, operation: &Operation) -> i32 {
    let change: i32 = match operation {
        Operation::Left(n) => -*n,
        Operation::Right(n) => *n
    };
    
    let mut hit_count: i32 = (change.abs() / 100) as i32;
    let new_index: i32 = (((position.0 + change) % 100) + 100) % 100;
    hit_count += (
        position.0 != 0 && 
        match operation {
            Operation::Left(_) => new_index > position.0 || new_index == 0,
            Operation::Right(_) => new_index < position.0
        }
    ) as i32;

    position.0 = new_index;
    hit_count
}

fn apply_sequence_and_count_zeroes(sequence: &PasswordSequence) -> i32 {
    let mut position = Position(50);
    let mut hit_count: i32 = 0;
    for operation in sequence {
        let new_hits = move_position(&mut position, operation);
        hit_count += new_hits;

        #[cfg(debug_assertions)]
        println!("Moved to {position:?}. {new_hits} new hits.");
    }
    hit_count
}

fn parse_input(run_config: &RunConfig) -> PasswordSequence {
    PasswordSequence(
        read_to_string(run_config.get_test_path()).unwrap().lines().map(|line| {
            let direction: char = line.chars().nth(0).unwrap();
            let magnitude: i32 = line[1..].parse::<i32>().unwrap();
            if magnitude == 0 {
                panic!("The magnitude can't be zero!");
            }
            match direction {
                'L' => Operation::Left(magnitude),
                'R' => Operation::Right(magnitude),
                _ => panic!("Expected the first letter to be L or R.")
            }
        }).collect()
    )
}

pub fn run(run_config: &RunConfig) -> i32 {
    let input = parse_input(run_config);
    let result = apply_sequence_and_count_zeroes(&input);
    result
}