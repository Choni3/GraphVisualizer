use crate::{
    algorithms::algorithm::{Algorithm, Snapshot},
    graph::graph::Graph,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub active_node: u32,
    pub distances: HashMap<u32, i32>,
    pub queue: VecDeque<u32>,
    pub log: Vec<String>,
}
#[derive(Debug, PartialEq)]
pub struct BFS {
    pub graph: Graph,
    pub runtime: Runtime,
}

impl BFS {
    pub fn new(graph: Graph, source: u32) -> Option<Self> {
        if !graph.nodes.contains_key(&source) {
            return None;
        }
        let distances = graph.nodes.keys().map(|&key| (key, -1)).collect();
        return Some(BFS {
            graph,
            runtime: Runtime {
                queue: VecDeque::new(),
                distances,
                active_node: 9999,
                log: Vec::new(),
            },
        });
    }
}
impl BFS {
    fn add_log_line(&mut self, data: String) {
        self.runtime.log.push(data);
    }
    fn bfs_loop(&mut self, source: u32) {
        self.runtime.queue.push_front(source);
        self.runtime.distances.insert(source, 0);
        self.runtime.active_node = source;

        while let Some(u) = self.runtime.queue.pop_front() {
            self.runtime.active_node = u;
            let u_distance = *self.runtime.distances.get(&u).expect("It exists");
            println!(
                "Tamos en el nodo {} con distancia: {}",
                self.runtime.active_node, u_distance
            );
            for v in self
                .graph
                .nodes
                .get(&u)
                .expect("It should exist")
                .runtime
                .edges
                .values()
            {
                let current_distance = *self
                    .runtime
                    .distances
                    .get(&v.runtime.node_end)
                    .expect("It exists");
                let weight = v.runtime.weight as i32;
                if current_distance == -1 || current_distance >= weight + u_distance {
                    self.runtime
                        .distances
                        .insert(v.runtime.node_end, weight + u_distance);
                    self.runtime.queue.push_back(v.runtime.node_end);
                }
            }
        }

        let final_distances: Vec<String> = self
            .runtime
            .distances
            .iter()
            .map(|(node, dist)| {
                format!(
                    "The distance from node {} to node: {} is {}",
                    source, node, dist
                )
            })
            .collect();

        for distance_log in final_distances {
            self.add_log_line(distance_log);
        }
    }
}
impl Algorithm<Runtime> for BFS {
    fn get_snapshot(&self) -> Snapshot<Runtime> {
        Snapshot {
            data: self.runtime.clone(),
            graph: self.graph.to_snapshot().unwrap(),
        }
    }

    fn run(&mut self) {
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

        // Start with the BFS
        self.bfs_loop(0);
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
    fn test_bfs_works_fine() -> Result<(), String> {
        let my_graph = setup_left_chain();
        let mut my_bfs = BFS::new(my_graph, 0).unwrap();

        my_bfs.bfs_loop(0);

        assert_eq!(my_bfs.runtime.distances.get(&0).unwrap().clone(), 0);
        assert_eq!(my_bfs.runtime.distances.get(&1).unwrap().clone(), 1);
        assert_eq!(my_bfs.runtime.distances.get(&2).unwrap().clone(), 2);
        assert_eq!(my_bfs.runtime.distances.get(&3).unwrap().clone(), 3);
        assert_eq!(my_bfs.runtime.distances.get(&4).unwrap().clone(), 4);
        assert_eq!(my_bfs.runtime.distances.get(&5).unwrap().clone(), 5);

        Ok(())
    }
}
