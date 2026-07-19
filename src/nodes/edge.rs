
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;

#[derive(Default)]
#[allow(dead_code)]
pub struct Edge {
    pub name: String,
    pub weight: u32,
    pub edges: Vec<RefCell<Vertex>>,
}
