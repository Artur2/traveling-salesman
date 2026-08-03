use crate::internal::types::MutableEdgeReference;
use crate::nodes::edge::Edge;
use crate::nodes::graph::Graph;
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
        for edge in self.edges.iter() {
            let borrowed = edge.borrow();
            formatted += edge.borrow().identifier.to_string().as_str();
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
            let mut has = false;

            if let Some(destination) = &borrowed_edge.destination {
                let borrowed_vector = destination.borrow();
                has |= borrowed_vector.name == destination_vector_name;
            }

            if let Some(source) = &borrowed_edge.source {
                let borrowed_vector = source.borrow();
                has |= borrowed_vector.name == destination_vector_name;
            }

            has
        })
    }

    #[allow(unused_assignments)]
    pub fn add_connection(
        graph: &Graph,
        source_vertex: &Rc<RefCell<Vertex>>,
        destination_vertex: &Rc<RefCell<Vertex>>,
        weight: u32,
    ) {
        let mut edge_name: String = Default::default();
        let mut source_vertex_name: String = Default::default();
        let mut destination_vertex_name: String = Default::default();
        {
            source_vertex_name += source_vertex.borrow().name.as_str();
            destination_vertex_name += destination_vertex.borrow().name.as_str();

            edge_name = format!("{}-{}", source_vertex_name, destination_vertex_name);
        }

        let mut new_edge = Edge::new(edge_name, weight);
        new_edge.source = Some(source_vertex.clone());
        new_edge.destination = Some(destination_vertex.clone());
        
        let new_edge_rc = Rc::new(RefCell::new(new_edge));

        let mut source_modify = false;
        let mut destination_modify = false;
        {
            if !source_vertex
                .borrow()
                .has_connection(&destination_vertex_name)
            {
                source_modify = true;
            }
            if !destination_vertex
                .borrow()
                .has_connection(&source_vertex_name)
            {
                destination_modify = true;
            }
        }

        if source_modify {
            let mut mutable_vertex = source_vertex.borrow_mut();
            mutable_vertex.edges.push(new_edge_rc.clone());
        }
        if destination_modify {
            let mut mutable_vertex = destination_vertex.borrow_mut();
            mutable_vertex.edges.push(new_edge_rc.clone());
        }
    }

    pub fn get_except_edge(&self, vertices: &Vec<String>) -> Vec<&MutableEdgeReference> {
        let edges = self
            .edges
            .iter()
            .filter(|e| {
                let borrowed_edge = e.borrow();
                let mut found = false;
                for vertex in vertices.iter() {
                    found |= !borrowed_edge.has_connection(&vertex);
                }

                found
            })
            .collect();

        edges
    }
}
