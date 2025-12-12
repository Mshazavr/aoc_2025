use std::collections::HashMap;
use std::fs::read_to_string;

use crate::RunConfig;

const PRESENT_DIM: usize = 3;
const NUM_PRESENT_TYPES: usize = 6;

#[derive(Debug)]
struct PresentType {
    shape_mask: [[bool; PRESENT_DIM]; PRESENT_DIM],
}
impl PresentType {
    fn from(text: &str) -> Self {
        Self {
            shape_mask: [
                [
                    text.chars().nth(0).unwrap() == '#',
                    text.chars().nth(1).unwrap() == '#',
                    text.chars().nth(2).unwrap() == '#',
                ],
                [
                    text.chars().nth(3).unwrap() == '#',
                    text.chars().nth(4).unwrap() == '#',
                    text.chars().nth(5).unwrap() == '#',
                ],
                [
                    text.chars().nth(6).unwrap() == '#',
                    text.chars().nth(7).unwrap() == '#',
                    text.chars().nth(8).unwrap() == '#',
                ],
            ],
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    present_types: Vec<PresentType>,
    board_shapes: Vec<(usize, usize)>,
    board_requirements: Vec<Vec<usize>>,
}
impl Puzzle {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split("\n\n").collect();
        let present_types: Vec<PresentType> = parts[0..(parts.len() - 1)]
            .iter()
            .map(|part| PresentType::from(part))
            .collect();

        let board_texts = parts[parts.len() - 1].split("\n");
        let mut board_shapes: Vec<(usize, usize)> = vec![];
        let mut board_requirements: Vec<Vec<usize>> = vec![];
        for board_text in board_texts {
            let board_text_parts: Vec<&str> = board_text.split(" ").collect();
            let shape_text_parts: Vec<&str> = board_text_parts[0].split("x").collect();
            board_shapes.push((
                shape_text_parts[0].parse::<usize>().unwrap(),
                shape_text_parts[1][0..(shape_text_parts[1].len() - 1)]
                    .parse::<usize>()
                    .unwrap(),
            ));
            board_requirements.push(
                board_text_parts[1..(board_text_parts.len() - 1)]
                    .iter()
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect(),
            );
        }

        Self {
            present_types,
            board_shapes,
            board_requirements,
        }
    }
}

fn parse_input(run_config: &RunConfig) -> Puzzle {
    Puzzle::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> usize {
    let puzzle = parse_input(run_config);
    println!("{puzzle:?}");
    0
}
