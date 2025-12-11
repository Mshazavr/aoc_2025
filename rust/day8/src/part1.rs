use std::fs::read_to_string;

use crate::RunConfig;

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
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

fn spread_dfs(
    node: usize,
    visited_map: &mut Vec<Option<i64>>,
    edges_mat: &Vec<Vec<usize>>,
    component_sizes: &mut Vec<i64>,
) {
    component_sizes[visited_map[node].unwrap() as usize] += 1;
    for neighbour in edges_mat[node].iter() {
        match visited_map[*neighbour] {
            None => {
                visited_map[*neighbour] = Some(visited_map[node].unwrap());
                spread_dfs(*neighbour, visited_map, edges_mat, component_sizes);
            }
            Some(_) => {}
        }
    }
}

fn get_num_components_after_merge(boxes: &Vec<JunctionBox>, num_connections: i64) -> i64 {
    let mut distances: Vec<(i64, (usize, usize))> = vec![];
    for (i, junction_box1) in boxes.iter().enumerate() {
        for (j, junction_box2) in boxes.iter().enumerate() {
            if j <= i {
                continue;
            }
            distances.push((
                junction_box_distance_squared(junction_box1, junction_box2),
                (i, j),
            ));
        }
    }

    distances.sort();

    let edges: Vec<(usize, usize)> = (0..num_connections)
        .map(|idx| distances[idx as usize].1)
        .collect();
    let mut edges_mat: Vec<Vec<usize>> = vec![vec![]; boxes.len()];
    for edge in edges {
        edges_mat[edge.0].push(edge.1);
        edges_mat[edge.1].push(edge.0);
    }

    let mut num_components: i64 = 0;
    let mut component_sizes: Vec<i64> = vec![];
    let mut visited_map: Vec<Option<i64>> = vec![None; boxes.len()];
    (0..boxes.len()).for_each(|node| {
        if visited_map[node].is_none() {
            visited_map[node] = Some(num_components);
            component_sizes.push(0);
            spread_dfs(node, &mut visited_map, &edges_mat, &mut component_sizes);
            num_components += 1;
        }
    });

    component_sizes.sort();
    component_sizes[(num_components - 1) as usize]
        * component_sizes[(num_components - 2) as usize]
        * component_sizes[(num_components - 3) as usize]
}

fn parse_input(run_config: &RunConfig) -> Vec<JunctionBox> {
    read_to_string(run_config.get_test_path())
        .unwrap()
        .lines()
        .map(|line| JunctionBox::from(line))
        .collect()
}

pub fn run(run_config: &RunConfig) -> i64 {
    let boxes = parse_input(run_config);
    get_num_components_after_merge(&boxes, 1000)
}
