use std::fs::read_to_string;

use crate::RunConfig;

#[derive(Clone)]
enum Operation {
    Plus,
    Mult
}

impl Operation {
    fn from(text: &str) -> Option<Self> {
        match text {
            "+" => Some(Operation::Plus),
            "*" => Some(Operation::Mult),
            _ => None
        }
    }
}

struct Problem {
    numbers: Vec<i64>,
    operation: Operation
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
    problems: Vec<Problem>
}

impl WorkSheet {
    fn from(text: &str) -> WorkSheet {
        let mut number_rows: Vec<Vec<i64>> = vec![];
        let mut operations: Vec<Operation> = vec![];
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0].chars().nth(0).unwrap().is_alphanumeric() {
                number_rows.push(parts.iter().map(|part| part.parse::<i64>().unwrap()).collect());
            }
            else {
                operations = parts.iter().map(|part| Operation::from(part).unwrap()).collect();
            }
        }

        
        let mut problems: Vec<Problem> = vec![];
        for (problem_idx, operation) in operations.iter().enumerate() {
            let problem = Problem {
                numbers: number_rows.iter().map(|number_row| number_row[problem_idx]).collect(),
                operation: operation.clone()
            };
            problems.push(problem);
        }

        Self {
            problems,
        }
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