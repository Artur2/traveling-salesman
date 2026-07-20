#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

type MutableVertexReferences = Vec<Rc<RefCell<Vertex>>>;
type EdgeReferences = Vec<Rc<RefCell<Edge>>>;

#[derive(Default)]
pub struct Graph {
    /// Name of graph
    pub name: String,
    /// Entry points to root vertices
    pub vertices: MutableVertexReferences,
    /// Flat vertex references
    pub vertex_references: MutableVertexReferences,
    /// Flat edge references
    pub edge_references: EdgeReferences,
}

impl Graph {
    pub fn new(name: String) -> Graph {
        Graph {
            name,
            vertices: vec![],
            vertex_references: vec![],
            edge_references: vec![],
        }
    }

    /// Only for disconnected vertices
    pub fn add_vertex(&mut self, name: String) {
        let found = self.vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        if found.is_none() {
            let new_vertex = Vertex::new(name);
            let cell = RefCell::new(new_vertex);
            let rc = Rc::new(cell);
            self.vertex_references.push(rc.clone());
            self.vertices.push(rc);
        } else {
            panic!("Vertex {} already exists", name);
        }
    }

    /// For connecting vertices to an existing edges, which connected to **root** vertices\
    /// **Panics** when cant find edge or edge already has destination
    pub fn connect_vertex_to_edge(&mut self, name: String, identifier: String) {
        let found_edge = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        if found_edge.is_none() {
            panic!("Create edge first")
        } else {
            if let Some(found_edge) = found_edge {
                if found_edge.borrow().destination.is_some() {
                    panic!("Edge already has source and destination");
                }
            }

            let vertex = Vertex::new(name);
            let vertex_rc = Rc::new(RefCell::new(vertex));

            let mut changing_edge = found_edge.unwrap().borrow_mut();

            changing_edge.destination = Some(vertex_rc.clone());
            self.vertices.push(vertex_rc.clone());
            self.vertex_references.push(vertex_rc.clone());
        }
    }

    /// For connecting edges to **root** vertices\
    /// **Panics** if edge with same identifier found\
    /// *Remarks*\
    /// If vertex not found, we create new root vertex
    pub fn connect_edge_to_vertex(&mut self, name: String, identifier: String, weight: u32) {
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
        if found_vertex.is_some() {
            let mut changing_vertex = found_vertex.unwrap().borrow_mut();
            let mut new_edge = Edge::new(identifier, weight);

            new_edge.source = Some(found_vertex.unwrap().clone());

            let new_edge_rc = Rc::new(RefCell::new(new_edge));
            self.edge_references.push(new_edge_rc.clone());
            changing_vertex.edges.push(new_edge_rc);
        } else {
            panic!("Create vertex first")
        }
    }

    /// Checking if graph has vertex by name
    pub fn has_vertex(&self, name: String) -> bool {
        let found = self.vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == name
        });

        found.is_some()
    }

    /// Checking if graph has edge by identifier
    pub fn has_edge(&self, identifier: String) -> bool {
        let found = self.edge_references.iter().find(|e| {
            let borrowed_e = e.borrow();
            borrowed_e.identifier == identifier
        });

        found.is_some()
    }

    /// Checking if edge has connections(source or/and destination or none)
    pub fn has_edge_connections(
        &self,
        identifier: String,
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
        let mut graph = Graph::default();
        graph.add_vertex(String::from("test"));

        assert!(graph.has_vertex(String::from("test")));
    }

    #[test]
    pub fn should_add_edge_to_vertex() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("vertex01"));
        graph.connect_edge_to_vertex(String::from("vertex01"), String::from("edge01"), 1);

        assert!(graph.has_edge(String::from("edge01")));
        assert!(graph.has_vertex(String::from("vertex01")));
    }

    #[test]
    pub fn should_add_child_vertex_to_edge() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("source"));
        graph.connect_edge_to_vertex(String::from("source"), String::from("edge01"), 1);
        graph.connect_vertex_to_edge(String::from("destination"), String::from("edge01"));

        assert!(graph.has_vertex(String::from("destination")));
    }

    #[test]
    #[should_panic(expected = "Create edge first")]
    pub fn should_panic_if_edge_is_not_exist() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("vertex01"));
        graph.connect_edge_to_vertex(String::from("vertex01"), String::from("edge01"), 1);

        graph.connect_vertex_to_edge(String::from("vertex01"), String::from("edge02"));
    }

    #[test]
    #[should_panic(expected = "Edge already has source and destination")]
    pub fn should_panic_if_edge_has_source_and_destination() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("vertex01"));
        graph.connect_edge_to_vertex(String::from("vertex01"), String::from("edge01"), 1);
        graph.connect_vertex_to_edge(String::from("vertex02"), String::from("edge01"));

        graph.connect_vertex_to_edge(String::from("vertex02"), String::from("edge01"));
    }

    #[test]
    pub fn edge_should_has_at_least_one_connection() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("vertex01"));
        graph.connect_edge_to_vertex(String::from("vertex01"), String::from("edge01"), 1);

        assert!(graph.has_edge_connections(String::from("edge01"), ConnectionType::Source));
    }

    #[test]
    #[should_panic(expected = "Vertex vertex01 already exists")]
    pub fn adding_duplicate_vertex_should_panic() {
        let mut graph = Graph::default();
        graph.add_vertex(String::from("vertex01"));
        graph.add_vertex(String::from("vertex01"));
    }

    #[test]
    #[should_panic(expected = "Create vertex first")]
    pub fn connecting_edge_to_unknown_vertex() {
        let mut graph = Graph::default();
        graph.connect_edge_to_vertex(String::from("vertex01"), String::from("edge01"), 1);
    }
}
