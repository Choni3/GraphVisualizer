use crate::edge::edge_model::EdgeSnapshot;
use crate::graph::graph::GraphSnapshot;
use crate::gui_graphs::gui_model::EdgeModel;
use crate::gui_graphs::gui_model::FinalGUIModel;
use crate::gui_graphs::gui_model::NodeModel;
use crate::gui_graphs::gui_model::draw_edge;
use crate::gui_graphs::gui_model::draw_node;
use iced::Point;
use iced::Size;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::column;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::{Column, button};
use iced::{Color, Element, Rectangle, Renderer, Theme};
use ordered_float::OrderedFloat;
use rand::prelude::*;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum NodeMessage {
    AddNode(u32, OrderedFloat<f64>, OrderedFloat<f64>),
    AddNodeSafe(u32, String, String),
    DeleteNode(u32),
    UpdateX(String),
    UpdateY(String),
    UpdateError(String),
    AddEdge(String, String),
}

#[derive(Clone)]
pub struct State {
    pub last_snapshot: GraphSnapshot,
    pub elements_data: HashMap<u32, FinalGUIModel>,
}
impl Default for State {
    fn default() -> Self {
        let pairs: Vec<(u32, FinalGUIModel)> = vec![
            (
                0,
                FinalGUIModel::Node(NodeModel {
                    position_x: OrderedFloat(100.0),
                    position_y: OrderedFloat(50.0),
                    radius: OrderedFloat(15.0),
                }),
            ),
            (
                1,
                FinalGUIModel::Node(NodeModel {
                    position_x: OrderedFloat(50.0),
                    position_y: OrderedFloat(100.0),
                    radius: OrderedFloat(15.0),
                }),
            ),
            (
                2,
                FinalGUIModel::Node(NodeModel {
                    position_x: OrderedFloat(150.0),
                    position_y: OrderedFloat(150.0),
                    radius: OrderedFloat(15.0),
                }),
            ),
            (
                10,
                FinalGUIModel::Edge(EdgeModel {
                    line_type: 1,
                    thickness: 5,
                }),
            ),
            (
                11,
                FinalGUIModel::Edge(EdgeModel {
                    line_type: 1,
                    thickness: 5,
                }),
            ),
        ];

        let map: HashMap<u32, FinalGUIModel> = pairs.into_iter().collect();
        State {
            elements_data: map,
            last_snapshot: GraphSnapshot {
                nodes: vec![0, 1, 2],
                edges: vec![
                    EdgeSnapshot {
                        node_start: 0,
                        node_end: 1,
                        id: 10,
                        weight: 20,
                    },
                    EdgeSnapshot {
                        node_start: 1,
                        node_end: 2,
                        id: 11,
                        weight: 20,
                    },
                ],
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct NewState {
    pub data: Vec<(OrderedFloat<f64>, OrderedFloat<f64>)>,
    pub drawn_nodes: HashMap<u32, (OrderedFloat<f64>, OrderedFloat<f64>)>,
    pub drawn_edges: Vec<(u32, u32)>,
}

#[derive(Default)]
pub struct GraphApp {
    pub counter: u32,
    pub state: State,
    pub drawn_nodes: HashMap<u32, (OrderedFloat<f64>, OrderedFloat<f64>)>,
    pub x_input: String,
    pub y_input: String,
    pub error_message: String,
}

fn draw_edges(state: &State, canvas: &mut canvas::Frame) -> Option<()> {
    let edges = state.last_snapshot.edges.clone();
    for edge in edges {
        let edge_model = state.elements_data.get(&edge.id)?.as_edge()?;
        let node_start = state.elements_data.get(&edge.node_start)?.as_node()?;
        let node_end = state.elements_data.get(&edge.node_end)?.as_node()?;
        draw_edge(canvas, edge_model, node_start, node_end);
    }
    Some(())
}
fn draw_nodes(state: &State, canvas: &mut canvas::Frame) -> Option<()> {
    let nodes = state.last_snapshot.nodes.clone();
    for node in nodes {
        let node_model = state.elements_data.get(&node)?.as_node()?;
        draw_node(canvas, node_model);
    }
    Some(())
}
impl<NodeMessage> canvas::Program<NodeMessage> for State {
    type State = ();
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        _: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, Size::new(800.0, 800.0));
        draw_nodes(self, &mut frame);
        draw_edges(self, &mut frame);
        vec![frame.into_geometry()]
    }
}
impl GraphApp {
    pub fn update(&mut self, message: NodeMessage) {
        // [TODO] Implement messages
    }
    pub fn view(&self) -> Column<NodeMessage> {
        let canvas: Element<NodeMessage> = canvas(self.state.clone()).into();
        column![canvas]
    }
}

// [TODO]: Add tests
