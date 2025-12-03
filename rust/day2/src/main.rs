mod part1;
mod part2;
mod common;

use common::RunConfig;

fn main() {
    let run_config = RunConfig::from_env();

    match run_config.part {
        1 => {
            let result = part1::run(&run_config);
            println!("Result: {result}!");
        }
        2 => {
            let result = part2::run(&run_config);
            println!("Result: {result}!");
        }
        _ => panic!("The part should either be 1 or 2")
    }

}
