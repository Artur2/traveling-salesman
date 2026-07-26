use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Vertex {
    pub name: String,
    pub edges: Vec<Rc<RefCell<Edge>>>,
}

impl Vertex {
    pub fn new(name: String) -> Self {
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

    pub fn add_connection(
        source_vertex: &Rc<RefCell<Vertex>>,
        destination_vertex: &Rc<RefCell<Vertex>>,
        weight: u32,
    ) {
        let mut edge_name: String = Default::default();
        {
            let destination_vertex_borrowed = destination_vertex.borrow();
            let source_vertex_borrowed = source_vertex.borrow();
            edge_name = format!("{}-{}", source_vertex_borrowed.name, destination_vertex_borrowed.name);
        }

        let mut new_edge = Edge::new(edge_name, weight);
        new_edge.source = Some(source_vertex.clone());
        new_edge.destination = Some(destination_vertex.clone());

        let edge_rc = Rc::new(RefCell::new(new_edge));
        source_vertex.borrow_mut().edges.push(edge_rc.clone());
        destination_vertex.borrow_mut().edges.push(edge_rc);
    }
}
