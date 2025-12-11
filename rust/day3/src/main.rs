mod common;
mod part1;
mod part2;

use common::RunConfig;

fn main() {
    let run_config = RunConfig::from_env();

    let result: u64 = match run_config.part {
        1 => part1::run(&run_config) as u64,
        2 => part2::run(&run_config),
        _ => panic!("The part should either be 1 or 2"),
    };

    println!("Result: {result}!");
}
