use core::panic;
use std::fs::read_to_string;

use crate::RunConfig;

struct Button {
    mask: usize,
}
impl Button {
    fn from(text: &str) -> Self {
        assert!(text.len() >= 2);
        let mask: usize = text[1..(text.len() - 1)]
            .split(",")
            .map(|num_str| 1 << num_str.parse::<usize>().unwrap())
            .sum();
        Self { mask }
    }
}

struct Machine {
    indicator_mask: usize,
    buttons: Vec<Button>,
}
impl Machine {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(" ").collect();
        assert!(parts.len() >= 3);

        let indicator_mask: usize = parts[0][1..(parts[0].len() - 1)]
            .chars()
            .enumerate()
            .map(|(idx, switch)| match switch {
                '.' => 0,
                '#' => 1 << idx,
                _ => panic!(),
            })
            .sum();

        let buttons = parts[1..(parts.len() - 1)]
            .iter()
            .map(|part| Button::from(part))
            .collect();

        Self {
            indicator_mask,
            buttons,
        }
    }

    fn get_min_button_presses(&self) -> usize {
        let mut min_num_pressed: Option<usize> = None;
        let num_buttons = self.buttons.len();
        for button_choice_mask in 0..(1 << num_buttons) {
            let mut num_pressed = 0;
            let mut final_indicator_mask = 0;
            for button_idx in 0..num_buttons {
                if (button_choice_mask >> button_idx) & 1 == 1 {
                    num_pressed += 1;
                    final_indicator_mask ^= self.buttons[button_idx].mask;
                }
            }
            if final_indicator_mask == self.indicator_mask {
                min_num_pressed = match min_num_pressed {
                    None => Some(num_pressed),
                    Some(min_num_pressed) => match min_num_pressed > num_pressed {
                        true => Some(num_pressed),
                        false => Some(min_num_pressed),
                    },
                }
            }
        }

        min_num_pressed.unwrap()
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
            .map(|machine| machine.get_min_button_presses())
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
