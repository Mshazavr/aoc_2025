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
        let mut nb_paper_count = 0;
        for i_diff in -1..2_isize {
            for j_diff in -1..2_isize {
                if i_diff == 0 && j_diff == 0 {
                    continue;
                }
                let (nb_row, nb_col) = (row_idx + i_diff, col_idx + j_diff);
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

    fn get_num_accessible_cells_and_remove(&mut self) -> usize {
        let mut num_accessible = 0;
        for row_idx in 0..self.rows.len() {
            for col_idx in 0..self.rows[0].len() {
                if let Cell::Empty = self.rows[row_idx][col_idx] {
                    continue;
                }

                if self.is_cell_accessible(row_idx as isize, col_idx as isize) {
                    num_accessible += 1;
                    self.rows[row_idx][col_idx] = Cell::Empty;
                }
            }
        }
        num_accessible
    }

    fn cascade_remove(&mut self) -> usize {
        let mut total_removed = 0;
        loop {
            let removed = self.get_num_accessible_cells_and_remove();
            total_removed += removed;
            if removed == 0 {
                break total_removed;
            }
        }
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
    let mut input_grid = parse_input(run_config);
    input_grid.cascade_remove()
}