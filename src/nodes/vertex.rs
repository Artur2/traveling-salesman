
use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Vertex<'a> {
    pub name: &'a str,
    pub edges: Vec<Rc<RefCell<Edge<'a>>>>,
}

impl<'a> Vertex<'a> {
    pub fn new(name: &'a str) -> Self {
        Vertex {
            name,
            edges: vec![]
        }
    }
}
