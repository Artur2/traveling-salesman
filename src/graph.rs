#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

type MutableVertexReferences<'a> = Vec<Rc<RefCell<Vertex<'a>>>>;
type MutableEdgeReferences<'a> = Vec<Rc<RefCell<Edge<'a>>>>;

#[derive(Default)]
pub struct Graph<'a> {
    /// Name of graph
    pub name: &'a str,
    /// Flat vertex references
    pub vertex_references: MutableVertexReferences<'a>,
    /// Flat edge references
    pub edge_references: MutableEdgeReferences<'a>,
}

impl<'a> Graph<'a> {
    pub fn new(name: &'a str) -> Self {
        Graph {
            name,
            vertex_references: vec![],
            edge_references: vec![],
        }
    }

    /// Only for disconnected vertices
    pub fn add_vertex(&mut self, name: &'a str) {
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

    pub fn connect_vertices(
        &mut self,
        source_vector_name: &'a str,
        destination_vector_name: &'a str,
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

        let mut source_vector = found_source_vector.unwrap().borrow_mut();
        let mut destination_vector = found_destination_vector.unwrap().borrow_mut();

        let mut edge = Edge::new(edge_identifier, weight);
        edge.source = Some(found_source_vector.unwrap().clone());
        edge.destination = Some(found_destination_vector.unwrap().clone());

        let edge_rc = Rc::new(RefCell::new(edge));
        source_vector.edges.push(edge_rc.clone());
        destination_vector.edges.push(edge_rc.clone());

        self.edge_references.push(edge_rc);
    }

    /// Checking if graph has vertex by name
    pub fn has_vertex(&self, name: &'a str) -> bool {
        let found = self.vertex_references.iter().find(|v| {
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

    pub fn has_connection_between_vertices(&self, source: &'a str, destination: &'a str) -> bool {
        self.edge_references.iter().any(|edge| {
            let borrowed_edge = edge.borrow();
            let unwrapped_source = borrowed_edge.source.as_ref().unwrap();
            let unwrapped_destination = borrowed_edge.destination.as_ref().unwrap();

            let borrowed_source = unwrapped_source.borrow();
            let borrowed_destination = unwrapped_destination.borrow();

            (borrowed_source.name == source && borrowed_destination.name == destination)
                || (borrowed_source.name == destination && borrowed_destination.name == source)
        })
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

    /// Generates random routes from source to destination
    /// **Remarks**
    /// You need to add enough vertices to obtain random ways
    pub fn generate_random_routes(
        &self,
        source: &'a str,
        destination: &'a str,
    ) -> Vec<MutableVertexReferences<'a>> {
        
        let mut stack = vec![];

        let starting_point = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == source
        });

        if starting_point.is_none() {
            panic!("Create vertex first");
        }

        let starting_rc = starting_point.unwrap();
        stack.push(starting_rc.clone());
        
        while !stack.is_empty() {
            // create walking vector
            // walk until dead end or destination
            // create another vector with walked if we meet different ways
        }

        todo!("Implement generate_random_routes")
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
    #[ignore]
    pub fn generate_random_paths_should_correctly_return_result() {
        let mut graph = Graph::new("test");
        graph.add_vertex("Polevskoy");
        graph.add_vertex("Revda");
        graph.add_vertex("Pervouralsk");
        graph.add_vertex("Ekaterinburg");
        graph.add_vertex("Sysert");

        graph.connect_vertices("Polevskoy", "Revda", "pore", 2);
        graph.connect_vertices("Revda", "Pervouralsk", "repe", 2);
        graph.connect_vertices("Pervouralsk", "Ekaterinburg", "pere", 4);
        graph.connect_vertices("Ekaterinburg", "Polevskoy", "ekpo", 5);
        graph.connect_vertices("Ekaterinburg", "Revda", "ekre", 3);
        graph.connect_vertices("Sysert", "Polevskoy", "sypo", 3);
        graph.connect_vertices("Ekaterinburg", "Sysert", "eksy", 3);

        todo!("Implement test");
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
