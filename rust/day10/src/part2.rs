use core::panic;
use std::fs::read_to_string;

use crate::RunConfig;

struct Button {
    mask: Vec<bool>,
}
impl Button {
    fn from(text: &str, length: usize) -> Self {
        assert!(text.len() >= 2);
        let mut mask: Vec<bool> = vec![false; length];
        text[1..(text.len() - 1)]
            .split(",")
            .for_each(|num_str| mask[num_str.parse::<usize>().unwrap()] = true);
        Self { mask }
    }
}

struct Joltage {
    levels: Vec<i64>
}
impl Joltage {
    fn from(text: &str) -> Self {
        assert!(text.len() >= 2);
        let levels: Vec<i64> = text[1..(text.len() - 1)]
            .split(",")
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        Self { levels }
    }
}

struct Machine {
    buttons: Vec<Button>,
    joltage: Joltage
}
impl Machine {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(" ").collect();
        assert!(parts.len() >= 3);

        let indicator_length: usize = parts[0].len() - 2;

        let buttons = parts[1..(parts.len() - 1)]
            .iter()
            .map(|part| Button::from(part, indicator_length))
            .collect();

        let joltage = Joltage::from(parts[parts.len() - 1]);

        Self {
            buttons,
            joltage,
        }
    }

    fn get_loss(&self, x: &Vec<i64>) -> i64 {
        // The loss is ||Ax - b||_1 + ||x||_1 
        // Where A is the button matrix and
        // Where b is the target joltage

        let mut loss: i64 = 0;
        let mut final_joltage: Vec<i64> = vec![0; self.joltage.levels.len()];
        
        for (i, x_i) in x.iter().enumerate() {
            loss += x_i.abs();
            for (j, on) in self.buttons[i].mask.iter().enumerate() {
                if *on {
                    final_joltage[j] += x_i;
                }
            }
        }
        
        for idx in 0..final_joltage.len() {
            loss += (final_joltage[idx] - self.joltage.levels[idx]).abs();
        }

        loss
    }

    fn get_min_button_presses(&self) -> usize {
        let mut x: Vec<i64> = vec![0; self.buttons.len()];

        loop {
            //println!("{x:?}");

            let mut min_new_x: Vec<i64> = x.clone();
            let mut min_new_loss: i64 = self.get_loss(&x);
            for b3_mask in 1..(3_usize.pow(x.len() as u32)) {
                let mut new_x = x.clone();
                for idx in 0..x.len() {
                    match (b3_mask / (3_usize.pow(idx as u32))) % 3 {
                        0 => {},
                        1 => { new_x[idx] += 1; },
                        2 => { new_x[idx] -= 1; },
                        _ => panic!()
                    }
                }

                // if new_x.iter().any(|x_i| *x_i < 0) {
                //    continue;
                // }

                let new_loss = self.get_loss(&new_x);
                if new_loss < min_new_loss {
                    min_new_loss = new_loss;
                    min_new_x = new_x;
                }
                
            }

            if x == min_new_x {
                break;
            }

            x = min_new_x;
        }

        self.get_loss(&x) as usize
    }
}

struct Manual {
    machines: Vec<Machine>,
}
impl Manual {
    fn from(text: &str) -> Self {
        let machines = text.lines().map(|line| Machine::from(line)).collect();
        Self { machines }
    }

    fn get_min_total_button_presses(&self) -> usize {
        self.machines
            .iter()
            .enumerate()
            .map(|(idx, machine)| {
                println!("Machine {}/{}", idx, self.machines.len());
                machine.get_min_button_presses()
            })
            .sum()
    }
}

fn parse_input(run_config: &RunConfig) -> Manual {
    Manual::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> usize {
    let manual = parse_input(run_config);
    manual.get_min_total_button_presses()
}


// 21329
// 21182