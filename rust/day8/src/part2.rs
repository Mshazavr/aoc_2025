use std::fs::read_to_string;

use crate::RunConfig;


struct JunctionBox {
    x: i64,
    y: i64,
    z: i64
}

impl JunctionBox {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(",").collect();
        Self {
            x: parts[0].parse::<i64>().unwrap(),
            y: parts[1].parse::<i64>().unwrap(),
            z: parts[2].parse::<i64>().unwrap(),
        }
    }
}

fn junction_box_distance_squared(box1: &JunctionBox, box2: &JunctionBox) -> i64 {
    (box1.x - box2.x).pow(2) + (box1.y - box2.y).pow(2) + (box1.z - box2.z).pow(2)
}

fn get_component_representative_id(node: usize, representative_ids: &mut Vec<usize>) -> usize{
    if representative_ids[node] != node {
        representative_ids[node] = get_component_representative_id(representative_ids[node], representative_ids);
    }
    representative_ids[node]
}

fn merge_components(node1: usize, node2: usize, component_sizes: &mut Vec<i64>, representative_ids: &mut Vec<usize>) {
    let mut smaller_node = node1;
    let mut bigger_node = node2;
    if component_sizes[node1] > component_sizes[node2] {
        smaller_node = node2;
        bigger_node = node1;
    }

    representative_ids[smaller_node] = bigger_node;
    component_sizes[bigger_node] += component_sizes[smaller_node];
}

fn get_the_x_product_of_the_critical_connection(boxes: &Vec<JunctionBox>) -> i64 {
    let mut distances: Vec<(i64, (usize, usize))> = vec![];
    for (i, junction_box1) in boxes.iter().enumerate() {
        for (j, junction_box2) in boxes.iter().enumerate() {
            if j <= i {
                continue;
            }
            distances.push((junction_box_distance_squared(junction_box1, junction_box2), (i, j)));
        }
    }

    distances.sort();

    let mut num_components: usize = boxes.len();
    let mut component_sizes: Vec<i64> = vec![1; boxes.len()];
    let mut representative_ids: Vec<usize> = (0..boxes.len()).collect();

    for (_, (junction_box1_idx, junction_box2_idx)) in distances {
        let representative_node1 = get_component_representative_id(junction_box1_idx, &mut representative_ids);
        let representative_node2 = get_component_representative_id(junction_box2_idx, &mut representative_ids);
        if representative_node1 == representative_node2 {
            continue;
        }

        merge_components(representative_node1, representative_node2, &mut component_sizes, &mut representative_ids);
        num_components -= 1;
        if num_components == 1 {
            return boxes[junction_box1_idx].x * boxes[junction_box2_idx].x;
        }
    }

    panic!();

}

fn parse_input(run_config: &RunConfig) -> Vec<JunctionBox> {
    read_to_string(run_config.get_test_path()).unwrap().lines().map(|line| JunctionBox::from(line)).collect()
}

pub fn run(run_config: &RunConfig) -> i64 {
    let boxes = parse_input(run_config);
    get_the_x_product_of_the_critical_connection(&boxes)
}