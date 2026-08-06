#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::internal::types::{MutableEdgeReferences, MutableVertexReferences};
use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use crate::upgrade_conditionally;
use rand::Rng;
use std::cell::RefCell;
use std::ops::Index;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct Graph {
    /// Name of graph
    pub name: String,
    /// Flat vertex references
    pub vertex_references: MutableVertexReferences,
    /// Flat edge references
    pub edge_references: MutableEdgeReferences,
}

impl Graph {
    pub(crate) fn new(name: String) -> Self {
        Graph {
            name,
            vertex_references: vec![],
            edge_references: vec![],
        }
    }

    /// Only for disconnected vertices
    pub(crate) fn add_vertex(&mut self, name: String) {
        let found = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        match found {
            None => {
                let new_vertex = Vertex::new(name);
                let cell = RefCell::new(new_vertex);
                let rc = Rc::new(cell);
                self.vertex_references.push(rc);
            }
            Some(_) => panic!("Vertex {} already exists", name),
        }
    }

    pub(crate) fn connect_vertices(
        &mut self,
        source_vector_name: String,
        destination_vector_name: String,
        weight: u32,
    ) {
        if self.has_connection_between_vertices(&source_vector_name, &destination_vector_name) {
            panic!("Duplicate connection");
        }

        let found_source_vector = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == source_vector_name
        });
        let found_destination_vector = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == destination_vector_name
        });

        if found_source_vector.is_none() || found_destination_vector.is_none() {
            panic!("Create vertex first");
        }

        let source_vector_reference = found_source_vector.unwrap();
        let destination_vector_reference = found_destination_vector.unwrap();

        let mut source_vector = source_vector_reference.borrow_mut();
        let mut destination_vector = destination_vector_reference.borrow_mut();
        let edge_identifier = format!("{}-{}", source_vector.name, destination_vector.name);
        let mut edge = Edge::new(edge_identifier.to_owned(), weight);
        edge.source = Some(source_vector_reference.clone());
        edge.destination = Some(destination_vector_reference.clone());
        let edge_rc = Rc::new(RefCell::new(edge));

        source_vector.edges.push(edge_rc.clone());
        destination_vector.edges.push(edge_rc.clone());

        self.edge_references.push(edge_rc);
    }

    /// Checking if graph has vertex by name
    pub(crate) fn has_vertex(&self, name: &str) -> bool {
        let found = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        found.is_some()
    }

    /// Checking if graph has edge by identifier
    pub(crate) fn has_edge(&self, identifier: &str) -> bool {
        let found = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        found.is_some()
    }

    pub(crate) fn has_connection_between_vertices(
        &self,
        source: &String,
        destination: &String,
    ) -> bool {
        self.edge_references.iter().map(|v| v.borrow()).any(|edge| {
            if edge.source.is_none() || edge.destination.is_none() {
                return false;
            }

            let source_rc = edge.source.as_ref().unwrap();
            let destination_rc = edge.destination.as_ref().unwrap();

            let borrowed_source = source_rc.borrow();
            let borrowed_destination = destination_rc.borrow();

            return (&borrowed_source.name == source && &borrowed_destination.name == destination)
                || (&borrowed_source.name == destination && &borrowed_destination.name == source);
        })
    }

    /// Checking if edge has connections(source or/and destination or none)
    pub fn has_edge_connections(&self, identifier: &str, connection_type: ConnectionType) -> bool {
        let found = self
            .edge_references
            .iter()
            .map(|v| v.borrow())
            .find(|e| e.identifier == identifier);

        if found.is_none() {
            return false;
        }

        let edge = found.unwrap();
        match connection_type {
            ConnectionType::None => edge.source.is_none() && edge.destination.is_none(),
            ConnectionType::Source => edge.source.is_some() && edge.destination.is_none(),
            ConnectionType::Destination => edge.source.is_none() && edge.destination.is_some(),
            ConnectionType::SourceAndDestination => {
                edge.source.is_some() && edge.destination.is_some()
            }
        }
    }
}

pub mod tests {
    use super::*;

    #[test]
    pub fn should_add_vertex() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("test".to_owned());
        assert!(graph.has_vertex("test"));
    }

    #[test]
    pub fn should_return_correct_result_with_edge_source_destination_connected() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("vertex01".to_owned());
        graph.add_vertex("vertex02".to_owned());
        graph.connect_vertices("vertex01".to_owned(), "vertex02".to_owned(), 1);

        assert!(
            graph.has_edge_connections("vertex01-vertex02", ConnectionType::SourceAndDestination)
        );
    }

    #[test]
    #[should_panic]
    pub fn should_panics_if_adding_duplicate_connections() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("vertex01".to_owned());
        graph.add_vertex("vertex02".to_owned());

        graph.connect_vertices("vertex01".to_owned(), "vertex02".to_owned(), 1);
        graph.connect_vertices("vertex01".to_owned(), "vertex02".to_owned(), 1);
    }

    #[test]
    #[should_panic]
    pub fn should_panics_if_adding_duplicate_connections_inverted() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("vertex01".to_owned());
        graph.add_vertex("vertex02".to_owned());

        graph.connect_vertices("vertex01".to_owned(), "vertex02".to_owned(), 1);
        graph.connect_vertices("vertex02".to_owned(), "vertex01".to_owned(), 1);
    }

    #[test]
    #[should_panic(expected = "Vertex vertex01 already exists")]
    pub fn adding_duplicate_vertex_should_panic() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("vertex01".to_owned());
        graph.add_vertex("vertex01".to_owned());
    }

    #[test]
    #[should_panic]
    pub fn connecting_edge_to_unknown_vertex() {
        let mut graph = Graph::new("test".to_owned());
        graph.connect_vertices("vertex01".to_owned(), "unknown_vertex".to_owned(), 1);
    }
}
