
use crate::nodes::edge::Edge;
use std::cell::RefCell;

#[derive(Default)]
#[allow(dead_code)]
pub struct Vertex {
    pub name: String,
    pub edges: Vec<RefCell<Edge>>,
}
