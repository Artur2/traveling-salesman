use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
#[allow(dead_code)]
pub struct Graph {
    pub name: String,
    pub vertices: Vec<RefCell<Vertex>>,
    pub vertices_references: Vec<Rc<RefCell<Vertex>>>,
    pub edges_references: Vec<Rc<RefCell<Edge>>>,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(name: String) -> Graph {
        Graph {
            name,
            vertices: vec![],
            vertices_references: vec![],
            edges_references: vec![],
        }
    }

    pub fn add_vertex(&mut self, name: String) {
        todo!()
    }

    pub fn add_vertex_to_edge(&mut self, name: String, weight: u32) {
        todo!()
    }

    pub fn add_edge_to_vertex(&mut self, name: String, weight: u32) {
        todo!()
    }
}
