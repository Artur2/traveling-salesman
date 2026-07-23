use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Vertex<'a> {
    pub name: &'a str,
    pub edges: Vec<Rc<RefCell<Edge<'a>>>>,
}

impl<'a> Vertex<'a> {
    pub fn new(name: &'a str) -> Self {
        Vertex {
            name,
            edges: vec![],
        }
    }

    pub fn has_connection(&self, destination_vector_name: &str) -> bool {
        self.edges.iter().any(|edge| {
            let borrowed_edge = edge.borrow();
            if let Some(destination) = borrowed_edge.destination.clone() {
                let borrowed_vector = destination.borrow();
                return borrowed_vector.name == destination_vector_name;
            }
            if let Some(source) = borrowed_edge.source.clone() {
                let borrowed_vector = source.borrow();
                return borrowed_vector.name == destination_vector_name;
            }

            return false;
        })
    }
}
