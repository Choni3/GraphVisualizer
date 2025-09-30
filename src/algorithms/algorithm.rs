use crate::graph::graph::GraphSnapshot;


pub struct Snapshot<T> {
	pub data: T,
	pub graph: GraphSnapshot
}
pub trait Algorithm<T> {
	fn run(&mut self);
	fn get_snapshot(&self) -> Snapshot<T>;
}