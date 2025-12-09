use std::fs::read_to_string;

use crate::RunConfig;

fn get_rectangle_area_from_corners(corner1: (i64, i64), corner2: (i64, i64)) -> i64{
    (corner1.0 - corner2.0 + 1).abs() * (corner1.1 - corner2.1 + 1).abs()
}

#[derive(Debug)]
struct Theatre {
    red_tile_locations: Vec<(i64, i64)>
}
impl Theatre {
    fn from(text: &str) -> Self {
        let red_tile_locations = text.lines().map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            (parts[0].parse::<i64>().unwrap(), parts[1].parse::<i64>().unwrap())
        }).collect();

        Self {
            red_tile_locations
        }
    }

    fn get_largest_rectangle_area(&self) -> i64 {
        let mut largest_area: i64 = 0;
        for corner1 in &self.red_tile_locations {
            for corner2 in &self.red_tile_locations {
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