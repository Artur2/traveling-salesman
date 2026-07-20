
use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Vertex {
    pub name: String,
    pub edges: Vec<Rc<RefCell<Edge>>>,
}

impl Vertex {
    pub fn new(name: String) -> Vertex {
        Vertex {
            name,
            edges: vec![]
        }
    }
}
