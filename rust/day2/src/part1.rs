use std::fs::read_to_string;
use std::cmp;

use crate::RunConfig;

struct Range {
    left: i64,
    right: i64
}

fn is_invalid_id(id: i64) -> bool{
    let mut id_copy = id;
    let mut num_digits = 0;
    while id_copy > 0 {
        num_digits += 1;
        id_copy /= 10;
    }
    num_digits = cmp::max(num_digits, 1);

    if num_digits % 2 == 1 {
        return false;
    }

    let first_half = id % (10_i64.pow(num_digits / 2));
    return id == first_half + first_half * 10_i64.pow(num_digits / 2); 
}


fn get_invalid_id_sum_in_range(range: Range) -> i64 {
    (range.left..(range.right+1)).map(
        |x| if is_invalid_id(x) { x } else { 0 }
    ).sum()
}


fn parse_input(run_config: &RunConfig) -> Vec<Range> {
    let line: String = read_to_string(run_config.get_test_path()).unwrap();
    let ranges = line.split(",").map(|range_string| {
        let endpoints: Vec<&str> = range_string.split("-").collect();
        Range{
            left: endpoints[0].parse::<i64>().unwrap(),
            right: endpoints[1].parse::<i64>().unwrap()
        }
    }).collect();
    ranges
}


pub fn run(run_config: &RunConfig) -> i64 {
    let input = parse_input(run_config);

    let result = input.into_iter().map(|range| get_invalid_id_sum_in_range(range)).sum();
    result
}