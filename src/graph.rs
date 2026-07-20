#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

type MutableVertexReferences<'a> = Vec<Rc<RefCell<Vertex<'a>>>>;
type EdgeReferences<'a> = Vec<Rc<RefCell<Edge<'a>>>>;

#[derive(Default)]
pub struct Graph<'a> {
    /// Name of graph
    pub name: &'a str,
    /// Entry points to root vertices
    pub vertices: MutableVertexReferences<'a>,
    /// Flat vertex references
    pub vertex_references: MutableVertexReferences<'a>,
    /// Flat edge references
    pub edge_references: EdgeReferences<'a>,
}

impl<'a> Graph<'a> {
    pub fn new(name: &'a str) -> Self {
        Graph {
            name,
            vertices: vec![],
            vertex_references: vec![],
            edge_references: vec![],
        }
    }

    /// Only for disconnected vertices
    pub fn add_vertex(&mut self, name: &'a str) {
        let found = self.vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        match found {
            None => {
                let new_vertex = Vertex::new(name);
                let cell = RefCell::new(new_vertex);
                let rc = Rc::new(cell);
                self.vertex_references.push(rc.clone());
                self.vertices.push(rc);
            }
            Some(_) => panic!("Vertex {} already exists", name),
        }
    }

    /// For connecting vertices to an existing edges, which connected to **root** vertices\
    /// **Panics** when cant find edge or edge already has destination
    pub fn connect_vertex_to_edge(&mut self, name: &'a str, identifier: &'a str) {
        let found_edge = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        match found_edge {
            Some(edge) => {
                let mut changing_edge = edge.borrow_mut();
                if changing_edge.destination.is_some() {
                    panic!("Edge already has source and destination");
                }
                let vertex = Vertex::new(name);
                let vertex_rc = Rc::new(RefCell::new(vertex));

                changing_edge.destination = Some(vertex_rc.clone());
                self.vertices.push(vertex_rc.clone());
                self.vertex_references.push(vertex_rc);
            }
            None => panic!("Create edge first"),
        }
    }

    /// For connecting edges to **root** vertices\
    /// **Panics** if edge with same identifier found\
    /// *Remarks*\
    /// If vertex not found, we create new root vertex
    pub fn connect_edge_to_vertex(&mut self, name: &'a str, identifier: &'a str, weight: u32) {
        let found_edge = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        if found_edge.is_some() {
            panic!("Already has edge with same identifier");
        }

        let found_vertex = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        // create vertex
        match found_vertex {
            Some(vertex) => {
                let mut changing_vertex = vertex.borrow_mut();
                let mut new_edge = Edge::new(identifier, weight);

                new_edge.source = Some(found_vertex.unwrap().clone());

                let edge_rc = Rc::new(RefCell::new(new_edge));
                self.edge_references.push(edge_rc.clone());
                changing_vertex.edges.push(edge_rc);
            }
            None => panic!("Create vertex first"),
        }
    }

    /// Checking if graph has vertex by name
    pub fn has_vertex(&self, name: &'a str) -> bool {
        let found = self.vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        found.is_some()
    }

    /// Checking if graph has edge by identifier
    pub fn has_edge(&self, identifier: &'a str) -> bool {
        let found = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        found.is_some()
    }

    /// Checking if edge has connections(source or/and destination or none)
    pub fn has_edge_connections(
        &self,
        identifier: &'a str,
        connection_type: ConnectionType,
    ) -> bool {
        let found = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        if found.is_none() {
            return false;
        }

        let edge = found.unwrap().borrow();
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
    pub fn should_add_edge_to_vertex() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.connect_edge_to_vertex("vertex01", "edge01", 1);

        assert!(graph.has_edge("edge01"));
        assert!(graph.has_vertex("vertex01"));
    }

    #[test]
    pub fn should_add_child_vertex_to_edge() {
        let mut graph = Graph::new("test");
        graph.add_vertex("source");
        graph.connect_edge_to_vertex("source", "edge01", 1);
        graph.connect_vertex_to_edge("destination", "edge01");

        assert!(graph.has_vertex("destination"));
    }

    #[test]
    #[should_panic(expected = "Create edge first")]
    pub fn should_panic_if_edge_is_not_exist() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.connect_edge_to_vertex("vertex01", "edge01", 1);

        graph.connect_vertex_to_edge("vertex01", "edge02");
    }

    #[test]
    #[should_panic(expected = "Edge already has source and destination")]
    pub fn should_panic_if_edge_has_source_and_destination() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.connect_edge_to_vertex("vertex01", "edge01", 1);
        graph.connect_vertex_to_edge("vertex02", "edge01");

        graph.connect_vertex_to_edge("vertex02", "edge01");
    }

    #[test]
    pub fn edge_should_has_at_least_one_connection() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.connect_edge_to_vertex("vertex01", "edge01", 1);

        assert!(graph.has_edge_connections("edge01", ConnectionType::Source));
    }

    #[test]
    #[should_panic(expected = "Vertex vertex01 already exists")]
    pub fn adding_duplicate_vertex_should_panic() {
        let mut graph = Graph::new("test");
        graph.add_vertex("vertex01");
        graph.add_vertex("vertex01");
    }

    #[test]
    #[should_panic(expected = "Create vertex first")]
    pub fn connecting_edge_to_unknown_vertex() {
        let mut graph = Graph::new("test");
        graph.connect_edge_to_vertex("vertex01", "edge01", 1);
    }
}
