use crate::{
    edge::edge_model::{Edge, EdgeSnapshot},
    node::node_model::{Node, NodeSnapshot},
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
#[derive(PartialEq, Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, Node>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct GraphSnapshot {
    pub nodes: Vec<u32>,
    pub edges: Vec<EdgeSnapshot>,
}

impl Default for GraphSnapshot {
    fn default() -> Self {
        GraphSnapshot {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
impl GraphSnapshot {
    pub fn vector_to_set<T>(lst: Vec<T>) -> HashSet<T>
    where
        T: Eq + Hash + Copy,
    {
        let mut set: HashSet<T> = HashSet::new();
        lst.iter().for_each(|node| {
            set.insert(*node);
            ()
        });
        set
    }
    pub fn set_to_vector<T>(set: HashSet<T>) -> Vec<T>
    where
        T: Eq + Hash + Copy,
    {
        set.iter().cloned().collect()
    }
    pub fn validate_and_reduce(graph_snapshot: GraphSnapshot) -> Result<GraphSnapshot, String> {
        let nodes_set = GraphSnapshot::vector_to_set(graph_snapshot.nodes.clone());

        let mut edges_nodes = graph_snapshot
            .edges
            .iter()
            .flat_map(|edge| vec![edge.node_start, edge.node_end]);
        let is_valid = edges_nodes.all(|cur_node| nodes_set.contains(&cur_node));

        if !is_valid {
            return Err(String::from("Sonsoni"));
        }
        let edges_unique = GraphSnapshot::set_to_vector(GraphSnapshot::vector_to_set(
            graph_snapshot.edges.clone(),
        ));

        Ok(GraphSnapshot {
            nodes: GraphSnapshot::set_to_vector(nodes_set),
            edges: edges_unique,
        })
    }
    pub fn from_node_snapshots(lst_nodes: Vec<NodeSnapshot>) -> Result<GraphSnapshot, String> {
        let merged_nodes: Vec<u32> = lst_nodes.iter().flat_map(|lst| lst.clone().nodes).collect();
        let merged_edges: Vec<EdgeSnapshot> =
            lst_nodes.iter().flat_map(|lst| lst.clone().edges).collect();

        GraphSnapshot::validate_and_reduce(GraphSnapshot {
            nodes: merged_nodes,
            edges: merged_edges,
        })
    }
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn to_snapshot(&self) -> Result<GraphSnapshot, String> {
        let nodes: Vec<NodeSnapshot> = self.nodes.values().map(|node| node.to_snapshot()).collect();
        GraphSnapshot::from_node_snapshots(nodes)
    }
    pub fn from_list_of_weighted_edges(edges: Vec<(u32, u32, u32)>, is_undirected: bool) -> Self {
        let mut current_graph = Self::new();
        for (u, v, w) in edges {
            if !current_graph.nodes.contains_key(&u) {
                current_graph.nodes.insert(u, Node::new_node_from_id(u));
            }
            if !current_graph.nodes.contains_key(&v) {
                current_graph.nodes.insert(v, Node::new_node_from_id(v));
            }

            current_graph
                .nodes
                .get_mut(&u)
                .unwrap()
                .runtime
                .edges
                .insert(v, Edge::new(u, v, w));

            if is_undirected {
                current_graph
                    .nodes
                    .get_mut(&v)
                    .unwrap()
                    .runtime
                    .edges
                    .insert(u, Edge::new(v, u, w));
            }
        }
        return current_graph;
    }
    pub fn from_list_of_edges(edges: Vec<(u32, u32)>, is_undirected: bool) -> Self {
        let weighted_edges = edges.iter().map(|(a, b)| (*a, *b, 1 as u32)).collect();
        Self::from_list_of_weighted_edges(weighted_edges, is_undirected)
    }

    pub fn add_directed_edge(&mut self, start: u32, end: u32) {
        if self.nodes.contains_key(&start) && self.nodes.contains_key(&end) {
            self.nodes
                .entry(start)
                .and_modify(|node| node.add_new_edge_to_node(end));
        }
    }
    pub fn add_undirected_edge(&mut self, u: u32, v: u32) {
        self.add_directed_edge(u, v);
        self.add_directed_edge(v, u);
    }
    pub fn add_new_node_to_graph(&mut self, id: u32, position_x: f64, position_y: f64) {
        self.nodes
            .entry(id)
            .or_insert(Node::new(id, position_x, position_y, 1.0));
    }

    pub fn remove_node_from_graph(&mut self, id: u32) {
        self.nodes.remove_entry(&id);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn has_unique_elements<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
        let mut seen = HashSet::new();
        vec.iter().all(|x| seen.insert(x))
    }
    fn setup_left_chain() -> Graph {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        self::Graph::from_list_of_edges(edges, false)
    }
    fn setup_right_chain() -> Graph {
        let edges = vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5)];
        self::Graph::from_list_of_edges(edges, false)
    }
    fn setup_undirected_right_chain() -> Graph {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        self::Graph::from_list_of_edges(edges, true)
    }
    fn setup_undirected_left_chain() -> Graph {
        let edges = vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5)];
        self::Graph::from_list_of_edges(edges, true)
    }

    fn setup_tree() -> Graph {
        let edges = vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (3, 7),
            (3, 8),
            (4, 9),
            (4, 10),
            (2, 5),
            (2, 6),
            (5, 11),
            (6, 12),
            (6, 13),
            (6, 14),
        ];
        self::Graph::from_list_of_edges(edges, false)
    }

    #[test]
    fn test_graph_chain_has_added_nodes() -> Result<(), String> {
        let my_graph = setup_left_chain();

        assert_eq!(
            my_graph.nodes.iter().len(),
            7,
            "all the chain nodes should be included"
        );
        Ok(())
    }
    #[test]
    fn test_graph_tree_has_desired_nodes() -> Result<(), String> {
        let my_graph = setup_tree();
        assert_eq!(
            my_graph.nodes.iter().len(),
            15,
            "all the chain nodes should be included"
        );
        Ok(())
    }

    #[test]
    fn test_graph_directed_chains_are_different() -> Result<(), String> {
        let direct = setup_left_chain();
        let reversed = setup_right_chain();
        assert_ne!(direct, reversed, "Chains are different");
        Ok(())
    }
    #[test]
    fn test_graph_undirected_chains_are_the_same() -> Result<(), String> {
        let direct = setup_undirected_right_chain();
        let reversed = setup_undirected_left_chain();
        assert_eq!(direct, reversed, "Both chains should be the same");
        Ok(())
    }

    #[test]
    fn test_graph_snapshot() -> Result<(), String> {
        let my_graph = setup_left_chain();
        let snapshot = my_graph.to_snapshot()?;

        assert!(
            has_unique_elements(&snapshot.nodes.clone()),
            "It should have deduplicated nodes"
        );
        Ok(())
    }

    #[test]
    fn test_sonsoni3() -> Result<(), String> {
        Ok(())
    }
    #[test]
    fn test_sonsoneos() -> Result<(), String> {
        let mut xx = 5;
        xx += 1;
        assert_eq!(xx, 6, "deben ser iguales {} and {}", xx, 6);
        Ok(())
    }
}
