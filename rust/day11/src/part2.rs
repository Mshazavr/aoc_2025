use std::collections::HashMap;
use std::fs::read_to_string;

use crate::RunConfig;

struct DAG {
    node_names: Vec<String>,
    edges_rev: Vec<Vec<usize>>,
    source_idx: usize,
    destination_idx: usize,
    intermediate1_idx: usize,
    intermediate2_idx: usize,
}
impl DAG {
    fn from(text: &str) -> Self {
        let mut node_names: Vec<String> = vec![];
        let mut node_names_to_idx_map: HashMap<String, usize> = HashMap::new();

        let rows: Vec<&str> = text.split("\n").collect();

        for row in rows.iter() {
            let parts: Vec<&str> = row.split(" ").collect();
            if !node_names_to_idx_map.contains_key(&parts[0][..(parts[0].len() - 1)]) {
                node_names_to_idx_map.insert(
                    String::from(&parts[0][..(parts[0].len() - 1)]),
                    node_names.len(),
                );
                node_names.push(String::from(&parts[0][..(parts[0].len() - 1)]));
            }
            parts[1..].into_iter().for_each(|node_name| {
                if !node_names_to_idx_map.contains_key(*node_name) {
                    node_names_to_idx_map.insert(String::from(*node_name), node_names.len());
                    node_names.push(String::from(*node_name));
                }
            });
        }

        let mut edges_rev: Vec<Vec<usize>> = vec![vec![]; node_names.len()];
        for row in rows {
            let parts: Vec<&str> = row.split(" ").collect();
            let source_idx = node_names_to_idx_map[&parts[0][..(parts[0].len() - 1)]];
            parts[1..].into_iter().for_each(|node_name| {
                let dest_idx = node_names_to_idx_map[*node_name];
                edges_rev[dest_idx].push(source_idx);
            });
        }

        let source_idx = node_names_to_idx_map["svr"];
        let destination_idx = node_names_to_idx_map["out"];
        let intermediate1_idx = node_names_to_idx_map["dac"];
        let intermediate2_idx = node_names_to_idx_map["fft"];

        Self {
            node_names,
            edges_rev,
            source_idx: source_idx,
            destination_idx: destination_idx,
            intermediate1_idx,
            intermediate2_idx,
        }
    }

    fn compute_num_paths_rec(
        &self,
        source_idx: usize,
        node_idx: usize,
        num_paths_into: &mut Vec<Option<usize>>,
    ) {
        num_paths_into[node_idx] = Some(1);

        if node_idx == source_idx {
            return;
        }

        let mut num_paths = 0;
        for neighbour in self.edges_rev[node_idx].iter() {
            if num_paths_into[*neighbour].is_none() {
                self.compute_num_paths_rec(source_idx, *neighbour, num_paths_into);
            }
            num_paths += num_paths_into[*neighbour].unwrap();
        }

        num_paths_into[node_idx] = Some(num_paths);
    }

    fn get_num_paths(&self) -> usize {
        let mut num_paths_into: Vec<Option<usize>> = vec![None; self.node_names.len()];
        let mut num_paths_from_int1_into: Vec<Option<usize>> = vec![None; self.node_names.len()];
        let mut num_paths_from_int2_into: Vec<Option<usize>> = vec![None; self.node_names.len()];

        self.compute_num_paths_rec(self.source_idx, self.destination_idx, &mut num_paths_into);
        self.compute_num_paths_rec(
            self.intermediate1_idx,
            self.destination_idx,
            &mut num_paths_from_int1_into,
        );
        self.compute_num_paths_rec(
            self.intermediate2_idx,
            self.destination_idx,
            &mut num_paths_from_int2_into,
        );

        num_paths_into[self.intermediate1_idx].unwrap_or(0)
            * num_paths_from_int1_into[self.intermediate2_idx].unwrap_or(0)
            * num_paths_from_int2_into[self.destination_idx].unwrap_or(0)
            + num_paths_into[self.intermediate2_idx].unwrap_or(0)
                * num_paths_from_int2_into[self.intermediate1_idx].unwrap_or(0)
                * num_paths_from_int1_into[self.destination_idx].unwrap_or(0)
    }
}

fn parse_input(run_config: &RunConfig) -> DAG {
    DAG::from(&read_to_string(run_config.get_test_path()).unwrap())
}

pub fn run(run_config: &RunConfig) -> usize {
    let manual = parse_input(run_config);
    manual.get_num_paths()
}
