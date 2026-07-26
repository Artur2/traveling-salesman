use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Vertex {
    pub name: String,
    pub edges: Vec<Rc<RefCell<Edge>>>,
}

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut formatted = format!("{} with edges: ", self.name);
        for (i, edge) in self.edges.iter().enumerate() {
            formatted += format!("{}", edge.borrow().identifier).as_str();
        }

        write!(f, "{}", formatted)
    }
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
            let mut has_in_destination = false;
            if let Some(destination) = borrowed_edge.destination.clone() {
                let borrowed_vector = destination.borrow();
                has_in_destination = borrowed_vector.name == destination_vector_name;
            }
            let mut has_in_source = false;
            if let Some(source) = borrowed_edge.source.clone() {
                let borrowed_vector = source.borrow();
                has_in_source = borrowed_vector.name == destination_vector_name;
            }

            return has_in_source || has_in_destination;
        })
    }

    #[allow(unused_assignments)]
    pub fn add_connection(
        source_vertex: &Rc<RefCell<Vertex>>,
        destination_vertex: &Rc<RefCell<Vertex>>,
        weight: u32,
    ) {
        let mut edge_name: String = Default::default();
        let mut source_vertex_name: String = Default::default();
        let mut destination_vertex_name: String = Default::default();
        {
            let destination_vertex_borrowed = destination_vertex.borrow();
            let source_vertex_borrowed = source_vertex.borrow();
            edge_name = format!(
                "{}-{}",
                source_vertex_borrowed.name, destination_vertex_borrowed.name
            );

            source_vertex_name = source_vertex_borrowed.name.clone();
            destination_vertex_name = destination_vertex_borrowed.name.clone();
        }

        let mut new_edge = Edge::new(edge_name, weight);
        new_edge.source = Some(source_vertex.clone());
        new_edge.destination = Some(destination_vertex.clone());

        let edge_rc = Rc::new(RefCell::new(new_edge));
        let mut source_modify = false;
        let mut destination_modify = false;
        {
            let source_vertex_mut = source_vertex.borrow();
            let destination_vertex_mut = destination_vertex.borrow();
            if !source_vertex_mut.has_connection(&destination_vertex_name) {
                source_modify = true;
            }
            if !destination_vertex_mut.has_connection(&source_vertex_name) {
                destination_modify = true;
            }
        }

        if source_modify {
            let mut source_vertex_mut = source_vertex.borrow_mut();
            source_vertex_mut.edges.push(edge_rc.clone());
        }
        if destination_modify {
            let mut destination_vertex_mut = destination_vertex.borrow_mut();
            destination_vertex_mut.edges.push(edge_rc.clone());
        }
    }
}
