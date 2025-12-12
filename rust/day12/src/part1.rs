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
        let mut first_relevant_idx = 0;
        while text.chars().nth(first_relevant_idx).unwrap() != '.'
            && text.chars().nth(first_relevant_idx).unwrap() != '#'
        {
            first_relevant_idx += 1;
        }

        let present_slice: &str = &text[first_relevant_idx..];

        Self {
            shape_mask: [
                [
                    present_slice.chars().nth(0).unwrap() == '#',
                    present_slice.chars().nth(1).unwrap() == '#',
                    present_slice.chars().nth(2).unwrap() == '#',
                ],
                [
                    present_slice.chars().nth(4).unwrap() == '#',
                    present_slice.chars().nth(5).unwrap() == '#',
                    present_slice.chars().nth(6).unwrap() == '#',
                ],
                [
                    present_slice.chars().nth(8).unwrap() == '#',
                    present_slice.chars().nth(9).unwrap() == '#',
                    present_slice.chars().nth(10).unwrap() == '#',
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
                board_text_parts[1..board_text_parts.len()]
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
    // println!("{puzzle:#?}");
    let mut possible = 0;
    for i in 0..puzzle.board_shapes.len() {
        let shape =&puzzle.board_shapes[i];
        let reqs = &puzzle.board_requirements[i];
        let v1 = shape.0 * shape.1;
        let v2 = 9 * reqs.iter().sum::<usize>();
        if v1 >= v2 {
            possible += 1;
        }
    }
    possible
}
