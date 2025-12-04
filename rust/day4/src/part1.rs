use std::fs::read_to_string;

use crate::RunConfig;

enum Cell {
    Paper,
    Empty
}

impl Cell {
    fn from(c: char) -> Option<Self> {
        match c {
            '@' => Some(Cell::Paper),
            '.' => Some(Cell::Empty),
            _ => None
        }
    }
}

struct Grid {
    rows: Vec<Vec<Cell>>
}

impl Grid {
    fn from(row_strings: Vec<String>) -> Self {
        Self {
            rows: row_strings.iter().map(|row_string| {
                row_string.chars().map(
                    |c| Cell::from(c).unwrap()
                ).collect()
            }).collect()
        }
    }

    fn is_cell_accessible(&self, row_idx: isize, col_idx: isize) -> bool {
        if let Cell::Empty = self.rows[row_idx as usize][col_idx as usize] {
            return false;
        }

        let mut nb_paper_count = 0;

        for row_diff in -1..2_isize {
            for col_diff in -1..2_isize {
                if row_diff == 0 && col_diff == 0 {
                    continue;
                }
                let (nb_row, nb_col) = (row_idx + row_diff, col_idx + col_diff);
                if 
                    nb_row >= 0 && nb_row < self.rows.len() as isize &&
                    nb_col >= 0 && nb_col < self.rows[0].len() as isize
                {
                    if let Cell::Paper = self.rows[nb_row as usize][nb_col as usize] {
                        nb_paper_count += 1;
                    }
                }
            }
        }
        nb_paper_count < 4
    }

    fn get_num_accessible_cells(&self) -> usize {
        (0..self.rows.len()).map(|row_idx| {
            (0..self.rows[0].len()).map(|col_idx| {
                self.is_cell_accessible(row_idx as isize, col_idx as isize) as usize
            }).sum::<usize>()
        }).sum()
    }
}


fn parse_input(run_config: &RunConfig) -> Grid {
    Grid::from(
        read_to_string(run_config.get_test_path())
        .unwrap()
        .lines()
        .map(|line| String::from(line)).collect()
    )
}

pub fn run(run_config: &RunConfig) -> usize {
    let input_grid = parse_input(run_config);
    input_grid.get_num_accessible_cells()
}