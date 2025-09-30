#[derive(Debug, Hash, Clone, Copy)]
pub struct GUIModel {
    pub line_type: u32,
    pub thickness: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EdgeSnapshot {
	pub id: u32,
    pub node_start: u32,
    pub node_end: u32,
    pub weight: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Runtime {
    pub node_start: u32,
    pub node_end: u32,
    pub weight: u32,
}
#[derive(Debug, Hash, Clone, Copy)]
pub struct Edge {
	pub id: u32,
    pub runtime: Runtime,
    pub gui_model: GUIModel,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        return self.runtime == other.runtime && self.id == other.id;
    }
}
impl Eq for Edge {}
impl Edge {
    pub fn new(node_start: u32, node_end: u32, weight: u32) -> Self {
        Edge {
			id: 0,
            runtime: Runtime {
                node_start,
                node_end,
                weight,
            },
            gui_model: GUIModel {
                line_type: 1,
                thickness: 1,
            },
        }
    }
	pub fn to_snapshot(&self) -> EdgeSnapshot {
		EdgeSnapshot {
			id: self.id,
			node_start: self.runtime.node_start,
			node_end: self.runtime.node_end,
			weight: self.runtime.weight
		}
	}
}
