#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use crate::types::{MutableEdgeReferences, MutableVertexReferences};
use rand::{Rng};
use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Graph<'a> {
    /// Name of graph
    pub name: &'a str,
    /// Flat vertex references
    pub vertex_references: MutableVertexReferences<'a>,
    /// Flat edge references
    pub edge_references: MutableEdgeReferences<'a>,
}

impl<'a> Graph<'a> {
    pub(crate) fn new(name: &'a str) -> Self {
        Graph {
            name,
            vertex_references: vec![],
            edge_references: vec![],
        }
    }

    /// Only for disconnected vertices
    pub(crate) fn add_vertex(&mut self, name: &'a str) {
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
        source_vector_name: &str,
        destination_vector_name: &str,
        edge_identifier: &'a str,
        weight: u32,
    ) {
        if self.has_connection_between_vertices(source_vector_name, destination_vector_name) {
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

        let mut edge = Edge::new(edge_identifier, weight);
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

    pub(crate) fn has_connection_between_vertices(&self, source: &str, destination: &str) -> bool {
        self.edge_references.iter().map(|v| v.borrow()).any(|edge| {
            if edge.source.is_none() || edge.destination.is_none() {
                return false;
            }

            let source_rc = edge.source.as_ref().unwrap();
            let destination_rc = edge.destination.as_ref().unwrap();

            let borrowed_source = source_rc.borrow();
            let borrowed_destination = destination_rc.borrow();

            (borrowed_source.name == source && borrowed_destination.name == destination)
                || (borrowed_source.name == destination && borrowed_destination.name == source)
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
        let mut graph = Graph::new("test");
        graph.add_vertex("test");
        assert!(graph.has_vertex("test"));
    }

    #[test]
    pub fn should_return_correct_result_with_edge_source_destination_connected() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.add_vertex("vertex02");
        graph.connect_vertices("vertex01", "vertex02", "edge01", 1);

        assert!(graph.has_edge_connections("edge01", ConnectionType::SourceAndDestination));
    }

    #[test]
    #[should_panic]
    pub fn should_panics_if_adding_duplicate_connections() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.add_vertex("vertex02");

        graph.connect_vertices("vertex01", "vertex02", "edge01", 1);
        graph.connect_vertices("vertex01", "vertex02", "edge01", 1);
    }

    #[test]
    #[should_panic]
    pub fn should_panics_if_adding_duplicate_connections_inverted() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.add_vertex("vertex02");

        graph.connect_vertices("vertex01", "vertex02", "asdasd", 1);
        graph.connect_vertices("vertex02", "vertex01", "test", 1);
    }

    #[test]
    #[should_panic(expected = "Vertex vertex01 already exists")]
    pub fn adding_duplicate_vertex_should_panic() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.add_vertex("vertex01");
    }

    #[test]
    #[should_panic]
    pub fn connecting_edge_to_unknown_vertex() {
        let mut graph = Graph::new("test");
        graph.connect_vertices("vertex01", "unknown_vertex", "edge01", 1);
    }
}
