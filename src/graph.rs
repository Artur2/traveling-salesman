use std::cell::RefCell;

use crate::nodes::vertex::Vertex;

#[derive(Default)]
#[allow(dead_code)]
pub struct Graph {
    pub name: String,
    pub vertices: Vec<RefCell<Vertex>>,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(name: String) -> Graph {
        Graph {
            name: name,
            vertices: vec![],
        }
    }
}
