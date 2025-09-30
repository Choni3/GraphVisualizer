use crate::edge::edge_model::{Edge, EdgeSnapshot};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct NodeSnapshot {
    pub nodes: Vec<u32>,
    pub edges: Vec<EdgeSnapshot>,
}

#[derive(PartialEq, Debug)]
pub struct Runtime {
    pub edges: HashMap<u32, Edge>,
}
impl Default for Runtime {
    fn default() -> Runtime {
        Runtime {
            edges: HashMap::new(),
        }
    }
}
#[derive(PartialEq, Debug)]
pub struct GUIModel {
    pub position_x: OrderedFloat<f64>,
    pub position_y: OrderedFloat<f64>,
    pub radius: OrderedFloat<f64>,
}
impl Default for GUIModel {
    fn default() -> GUIModel {
        GUIModel {
            position_x: OrderedFloat(0.0),
            position_y: OrderedFloat(0.0),
            radius: OrderedFloat(1.0),
        }
    }
}
#[derive(PartialEq, Debug, Default)]
pub struct Node {
    pub id: u32,
    pub runtime: Runtime,
    pub gui_model: GUIModel,
}
impl Node {
    pub fn new(id: u32, position_x: f64, position_y: f64, radius: f64) -> Self {
        Node {
            id,
            runtime: Runtime::default(),
            gui_model: GUIModel {
                position_x: OrderedFloat(position_x),
                position_y: OrderedFloat(position_y),
                radius: OrderedFloat(radius),
            },
        }
    }
    pub fn new_node_from_id(id: u32) -> Self {
        Node {
            id,
            ..Node::default()
        }
    }
    pub fn to_snapshot(&self) -> NodeSnapshot {
        NodeSnapshot {
            nodes: vec![self.id],
            edges: self
                .runtime
                .edges
                .values()
                .map(|edge| edge.to_snapshot())
                .collect(),
        }
    }
}
impl Node {
    pub fn create_new(id: u32, x: f64, y: f64) -> Self {
        Self::new(id, x, y, 1.0)
    }
    pub fn add_new_edge_to_node(&mut self, id: u32) {
        self.runtime
            .edges
            .entry(id)
            .or_insert(Edge::new(self.id, id, 1));
    }
    pub fn add_new_edge_to_node_with_weight(&mut self, id: u32, weight: u32) {
        self.runtime
            .edges
            .entry(id)
            .or_insert(Edge::new(self.id, id, weight));
    }
    pub fn remove_edge_from_node(&mut self, id: u32) {
        if !self.runtime.edges.contains_key(&id) {
            panic!("non-existent edge");
        }
        self.runtime.edges.remove_entry(&id);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn test_node_default() -> Result<(), String> {
        let node = Node::new_node_from_id(10);
        assert_eq!(node.gui_model.radius, OrderedFloat(1.0));
        Ok(())
    }

    #[test]
    fn test_node_adds_edge_correctly() -> Result<(), String> {
        let mut node_1 = self::Node::create_new(0, 10.0, 20.0);

        node_1.add_new_edge_to_node_with_weight(1, 10);
        let edge_expected = self::Edge::new(0, 1, 10);
        if let Some(value) = node_1.runtime.edges.get(&1) {
            println!("sonsaso");
            assert_eq!(*value, edge_expected);
        } else {
            panic!("key not found");
        }

        Ok(())
    }
    #[test]
    fn test_node_has_not_edges() -> Result<(), String> {
        let node_1 = self::Node::create_new(0, 10.0, 20.0);
        assert_eq!(
            node_1.runtime.edges.values().len(),
            0,
            "List of edges should be empty"
        );
        Ok(())
    }
    #[test]
    fn test_node_add_and_remove_edges() -> Result<(), String> {
        let mut node_1 = self::Node::create_new(0, 10.0, 20.0);
        node_1.add_new_edge_to_node_with_weight(1, 10);
        node_1.remove_edge_from_node(1);
        assert_eq!(
            node_1.runtime.edges.values().len(),
            0,
            "List of edges should be empty"
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_node_remove_non_existent_edge() {
        let mut node_1 = self::Node::create_new(0, 10.0, 20.0);
        node_1.add_new_edge_to_node_with_weight(1, 10);
        node_1.remove_edge_from_node(3);
        assert_eq!(
            node_1.runtime.edges.values().len(),
            0,
            "List of edges should be empty"
        );
    }

    #[test]
    fn test_node_snapshot_works_fine() {
        let mut node_1 = self::Node::create_new(0, 10.0, 20.0);
        node_1.add_new_edge_to_node_with_weight(1, 10);
        node_1.add_new_edge_to_node_with_weight(2, 100);
        node_1.add_new_edge_to_node_with_weight(3, 100);
        node_1.add_new_edge_to_node_with_weight(4, 100);

        let node_snapshot = node_1.to_snapshot();
        assert_eq!(
            node_snapshot.nodes,
            vec![0],
            "The snapshot should contain its own id"
        );

        let edges_snapshot_end: Vec<u32> = node_snapshot
            .edges
            .iter()
            .map(|edge| edge.node_end)
            .collect();
        print!("{:?}", node_snapshot.edges);
        assert!(
            node_snapshot.edges.iter().all(|edge| edge.node_start == 0),
            "The snapshot node_starts should be the own id"
        );
        let final_list: HashSet<u32> = vec![1, 2, 3, 4].iter().cloned().collect();

        assert_eq!(
            edges_snapshot_end.iter().cloned().collect::<HashSet<_>>(),
            final_list,
            "The list of edges should be the same"
        );
    }
}
