use std::fs::read_to_string;

use crate::RunConfig;

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Mult,
}

impl Operation {
    fn from(text: &str) -> Option<Self> {
        match text {
            "+" => Some(Operation::Plus),
            "*" => Some(Operation::Mult),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Plus => self.numbers.iter().sum(),
            Operation::Mult => {
                let mut acc = 1;
                for number in &self.numbers {
                    acc *= number;
                }
                acc
            }
        }
    }
}

struct WorkSheet {
    problems: Vec<Problem>,
}

impl WorkSheet {
    fn from(text: &str) -> WorkSheet {
        let grid: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
        let num_cols = grid[0].len();
        let num_rows = grid.len();

        let mut problems: Vec<Problem> = vec![];
        let mut cur_numbers: Vec<i64> = vec![];
        let mut cur_operation: Option<Operation> = None;
        for col_idx in 0..num_cols {
            let last_char = grid[num_rows - 1][col_idx];

            if last_char == '+' || last_char == '*' {
                if let Some(_) = cur_operation {
                    problems.push(Problem {
                        numbers: cur_numbers.clone(),
                        operation: cur_operation.unwrap(),
                    });
                }
                cur_numbers = vec![];
                cur_operation = Operation::from(format!("{last_char}").as_str());
            }

            let mut base: i64 = 1;
            let mut cur_number: Option<i64> = None;
            for row_idx in (0..num_rows).rev() {
                let cur_char = grid[row_idx][col_idx];
                if cur_char.is_alphanumeric() {
                    let diff: i64 = base * (cur_char.to_digit(10).unwrap() as i64);
                    cur_number = match cur_number {
                        None => Some(diff),
                        Some(x) => Some(x + diff),
                    };
                    base *= 10;
                }
            }
            if let Some(x) = cur_number {
                cur_numbers.push(x);
            }
        }
        problems.push(Problem {
            numbers: cur_numbers.clone(),
            operation: cur_operation.unwrap(),
        });

        Self { problems }
    }

    fn solve(&self) -> i64 {
        self.problems.iter().map(|problem| problem.solve()).sum()
    }
}

fn parse_input(run_config: &RunConfig) -> WorkSheet {
    WorkSheet::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> i64 {
    let worksheet = parse_input(run_config);
    worksheet.solve()
}
