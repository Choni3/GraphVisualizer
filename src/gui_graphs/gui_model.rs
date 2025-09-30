use iced::{Color, Point, widget::canvas};
use ordered_float::OrderedFloat;

pub trait GUIFriendly {
    fn update_gui(&self, frame: &mut canvas::Frame);
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct EdgeModel {
    pub line_type: u32,
    pub thickness: u32,
}
impl Default for EdgeModel {
    fn default() -> EdgeModel {
        EdgeModel {
            line_type: 0,
            thickness: 10,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NodeModel {
    pub position_x: OrderedFloat<f64>,
    pub position_y: OrderedFloat<f64>,
    pub radius: OrderedFloat<f64>,
}
impl Default for NodeModel {
    fn default() -> NodeModel {
        NodeModel {
            position_x: OrderedFloat(0.0),
            position_y: OrderedFloat(0.0),
            radius: OrderedFloat(1.0),
        }
    }
}

#[derive(Clone)]
pub enum FinalGUIModel {
    Node(NodeModel),
    Edge(EdgeModel),
}
impl FinalGUIModel {
    pub fn as_node(&self) -> Option<&NodeModel> {
        if let FinalGUIModel::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    pub fn as_edge(&self) -> Option<&EdgeModel> {
        if let FinalGUIModel::Edge(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

fn node_to_point(node: &NodeModel) -> Point {
    Point::new(
        node.position_x.into_inner() as f32,
        node.position_y.into_inner() as f32,
    )
}
pub fn draw_node(frame: &mut canvas::Frame, node: &NodeModel) {
    let node_circle = canvas::Path::circle(node_to_point(node), node.radius.into_inner() as f32);
    frame.fill(&node_circle, Color::WHITE);
}
pub fn draw_edge(frame: &mut canvas::Frame, edge: &EdgeModel, node_start:  &NodeModel, node_end: &NodeModel) {
    let line = canvas::Path::line(node_to_point(&node_start), node_to_point(&node_end));
    frame.stroke(
        &line,
        canvas::Stroke {
            width: edge.thickness as f32,
            style: canvas::Style::Solid(Color::WHITE),
            ..Default::default()
        },
    );
}

// fn draw_element(frame: &mut canvas::Frame, element: FinalGUIModel) {
//     match element {
//         FinalGUIModel::Node(node_element) => draw_node(frame, &node_element),
//         FinalGUIModel::Edge(edge_element) => draw_edge(frame, &edge_element),
//     }
// }
