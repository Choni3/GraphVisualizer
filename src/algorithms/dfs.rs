use crate::algorithms::algorithm::Snapshot;
use crate::{algorithms::algorithm::Algorithm, graph::graph::Graph};
use std::collections::HashMap;
use std::{thread, time};
#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub active_node: u32,
    pub distances: HashMap<u32, i32>,
    pub log: Vec<String>,
}
#[derive(Debug, PartialEq)]
pub struct DFS {
    pub graph: Graph,
    pub runtime: Runtime,
}

impl DFS {
    pub fn new(graph: Graph, source: u32) -> Option<Self> {
        if !graph.nodes.contains_key(&source) {
            return None;
        }
        let distances = graph.nodes.keys().map(|&key| (key, -1)).collect();
        return Some(DFS {
            graph,
            runtime: Runtime {
                log: Vec::new(),
                active_node: 9999,
                distances,
            },
        });
    }
}
impl DFS {
    fn dfs_recursion(&mut self, u: u32, parent: u32) -> u32 {
        self.runtime.active_node = u;
        self.runtime.distances.insert(u, 0);
        let neighbors: Vec<u32> = self
            .graph
            .nodes
            .get(&u)
            .expect("Should exist")
            .runtime
            .edges
            .keys()
            .copied()
            .collect();
        println!(
            "Current node: {} and parent {} and neighbors {:?}",
            u, parent, neighbors
        );
        thread::sleep(time::Duration::from_millis(10));
        let mut count: u32 = 1;
        for id in neighbors {
            // println!("Current neighbor: {}", id);
            if *self.runtime.distances.get(&id).unwrap() != -1 as i32 || id == parent {
                continue;
            }
            println!("Current neighbor: {}", id);
            count += self.dfs_recursion(id, u);
        }
        count
    }
    fn dfs_loop(&mut self, u: u32) -> u32 {
        self.runtime.distances.insert(u, 0);
        self.dfs_recursion(u, u)
    }
    fn add_log_line(&mut self, data: String) {
        self.runtime.log.push(data);
    }
}
impl Algorithm<Runtime> for DFS {
    fn get_snapshot(&self) -> Snapshot<Runtime> {
        Snapshot {
            data: self.runtime.clone(),
            graph: self.graph.to_snapshot().unwrap(),
        }
    }
    fn run(&mut self) {
        // Graph description for logs
        let node_indexes: Vec<u32> = self.graph.nodes.keys().cloned().collect();
        for id in node_indexes {
            self.add_log_line(format!("the graph has a node: {}", id));
        }
        let nodes: Vec<String> = self
            .graph
            .nodes
            .values()
            .map(|node| {
                let edge_keys: Vec<&u32> = node.runtime.edges.keys().collect();
                format!("Node {} with neighbors {:?}", node.id, edge_keys)
            })
            .collect();
        for node_log in nodes {
            self.add_log_line(node_log);
        }

        // Start with the DFS
        self.dfs_loop(0);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_left_chain() -> Graph {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        Graph::from_list_of_edges(edges, false)
    }

    #[test]
    fn test_dfs_works_fine() -> Result<(), String> {
        let my_graph = setup_left_chain();
        let mut my_dfs = DFS::new(my_graph, 0).unwrap();

        let result = my_dfs.dfs_loop(0);
		

		assert_eq!(7, result, "Number of nodes should be {}", 7);

        assert!(
            my_dfs
                .runtime
                .distances
                .values()
                .all(|distance| *distance != -1),
            "All the nodes should have been visited"
        );

        Ok(())
    }
}
