use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

use crate::RunConfig;

fn get_rectangle_area_from_corners(corner1: (i64, i64), corner2: (i64, i64)) -> i64 {
    ((corner1.0 - corner2.0).abs() + 1) * ((corner1.1 - corner2.1).abs() + 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Red,
    Empty,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn from(stops: Vec<(i64, i64)>) -> Self {
        let height: usize = stops.iter().map(|stop| stop.0 as usize).max().unwrap() + 2;
        let width: usize = stops.iter().map(|stop| stop.1 as usize).max().unwrap() + 2;
        let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Empty; width]; height];

        let mut marked_map: Vec<Vec<bool>> = vec![vec![false; width]; height];
        for i in 0..stops.len() {
            let mut start = stops[i];
            let end;
            if i + 1 == stops.len() {
                end = stops[0];
            } else {
                end = stops[i + 1];
            }

            let diff;
            if start.0 == end.0 {
                if start.1 < end.1 {
                    diff = (0, 1)
                } else {
                    diff = (0, -1)
                }
            } else {
                if start.0 < end.0 {
                    diff = (1, 0)
                } else {
                    diff = (-1, 0)
                }
            }

            while start != end {
                marked_map[start.0 as usize][start.1 as usize] = true;
                cells[start.0 as usize][start.1 as usize] = Cell::Red;

                start = (start.0 + diff.0, start.1 + diff.1);
            }
            marked_map[start.0 as usize][start.1 as usize] = true;
            cells[start.0 as usize][start.1 as usize] = Cell::Red;
        }

        let mut queue: VecDeque<(i64, i64)> = VecDeque::new();

        marked_map[0][0] = true;
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let diffs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for diff in diffs {
                let (new_x, new_y) = (x + diff.0, y + diff.1);
                if new_x < 0
                    || new_x >= marked_map.len() as i64
                    || new_y < 0
                    || new_y >= marked_map[0].len() as i64
                {
                    continue;
                }
                if !marked_map[new_x as usize][new_y as usize] {
                    marked_map[new_x as usize][new_y as usize] = true;
                    queue.push_back((new_x, new_y));
                }
            }
        }

        for i in 0..height {
            for j in 0..width {
                if !marked_map[i][j] {
                    cells[i][j] = Cell::Red;
                }
            }
        }

        Self { cells }
    }

    fn is_area_red(&self, corner1: (usize, usize), corner2: (usize, usize)) -> bool {
        let min_x = min(corner1.0, corner2.0);
        let max_x = max(corner1.0, corner2.0);
        let min_y = min(corner1.1, corner2.1);
        let max_y = max(corner1.1, corner2.1);
        for x in min_x..(max_x + 1) {
            for y in min_y..(max_y + 1) {
                if self.cells[x][y] == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug)]
struct Theatre {
    red_tile_locations: Vec<(i64, i64)>,
    coordinate_map_x: HashMap<i64, i64>,
    coordinate_map_y: HashMap<i64, i64>,
    mapped_grid: Grid,
}
impl Theatre {
    fn from(text: &str) -> Self {
        let red_tile_locations: Vec<(i64, i64)> = text
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(",").collect();
                (
                    parts[1].parse::<i64>().unwrap(),
                    parts[0].parse::<i64>().unwrap(),
                )
            })
            .collect();

        let mut coordinate_map_x: HashMap<i64, i64> = HashMap::new();
        let mut coordinate_map_y: HashMap<i64, i64> = HashMap::new();

        let mut x_coordinates: Vec<i64> = red_tile_locations.iter().map(|loc| loc.0).collect();
        let mut y_coordinates: Vec<i64> = red_tile_locations.iter().map(|loc| loc.1).collect();
        x_coordinates.sort();
        y_coordinates.sort();

        let mut next_mapped_x = 1;
        for (i, x) in x_coordinates.iter().enumerate() {
            if i == 0 || x_coordinates[i - 1] != *x {
                coordinate_map_x.insert(*x, next_mapped_x);
                next_mapped_x += 1;
            }
        }

        let mut next_mapped_y = 1;
        for (i, y) in y_coordinates.iter().enumerate() {
            if i == 0 || y_coordinates[i - 1] != *y {
                coordinate_map_y.insert(*y, next_mapped_y);
                next_mapped_y += 1;
            }
        }

        let mapped_grid = Grid::from(
            red_tile_locations
                .iter()
                .map(|location| {
                    (
                        *coordinate_map_x.get(&location.0).unwrap(),
                        *coordinate_map_y.get(&location.1).unwrap(),
                    )
                })
                .collect(),
        );

        Self {
            red_tile_locations,
            coordinate_map_x,
            coordinate_map_y,
            mapped_grid,
        }
    }

    fn get_largest_rectangle_area(&self) -> i64 {
        let mut largest_area: i64 = 0;
        for corner1 in &self.red_tile_locations {
            for corner2 in &self.red_tile_locations {
                let mapped_corner1 = (
                    *self.coordinate_map_x.get(&corner1.0).unwrap() as usize,
                    *self.coordinate_map_y.get(&corner1.1).unwrap() as usize,
                );
                let mapped_corner2 = (
                    *self.coordinate_map_x.get(&corner2.0).unwrap() as usize,
                    *self.coordinate_map_y.get(&corner2.1).unwrap() as usize,
                );

                if !self.mapped_grid.is_area_red(mapped_corner1, mapped_corner2) {
                    continue;
                }

                let candidate_area = get_rectangle_area_from_corners(*corner1, *corner2);
                if largest_area < candidate_area {
                    largest_area = candidate_area;
                }
            }
        }
        largest_area
    }
}

fn parse_input(run_config: &RunConfig) -> Theatre {
    Theatre::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> i64 {
    let theatre = parse_input(run_config);
    theatre.get_largest_rectangle_area()
}
