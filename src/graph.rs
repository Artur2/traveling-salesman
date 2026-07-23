#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nodes::connection_type::ConnectionType;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use rand::{Rng, random};
use std::cell::RefCell;
use std::ops::Index;
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
    pub fn has_edge_connections(
        &self,
        identifier: &'a str,
        connection_type: ConnectionType,
    ) -> bool {
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

    /// Generates random routes from source to destination
    /// **Remarks**
    /// You need to add enough vertices to obtain random ways
    /// TODO: Move to another struct
    pub fn generate_random_routes(
        &self,
        source: &'a str,
        destination: &'a str,
        amount_of_generations: i32
    ) -> Vec<MutableVertexReferences<'a>> {
        let starting_point = self.vertex_references.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == source
        });

        if starting_point.is_none() {
            panic!("Create vertex first");
        }

        let mut random_paths = vec![];

        for _ in 0..amount_of_generations {
            if let Some(path) =
                Self::generate_random_route(starting_point.unwrap().clone(), destination)
            {
                random_paths.push(path)
            }
        }

        random_paths
    }

    /// TODO: Move to another struct and check visited vertices instead of edges
    fn generate_random_route<'b>(
        starting_point: Rc<RefCell<Vertex<'b>>>,
        destination_identity: &'b str,
    ) -> Option<Vec<Rc<RefCell<Vertex<'b>>>>> {
        let mut stack = vec![];
        let mut visited_vertices = vec![];
        let mut visited_edges: Vec<&str> = vec![];
        let starting_rc = starting_point;

        stack.push(starting_rc.clone());

        while !stack.is_empty() {
            if let Some(current) = stack.pop() {
                let vertex = current.borrow();
                visited_vertices.push(current.clone());

                if vertex.name == destination_identity {
                    return Some(visited_vertices);
                }

                if vertex.edges.len() == 0 {
                    // vertex disconnected?
                    return None;
                }

                if vertex.edges.len() == 1 {
                    let edge = vertex.edges.index(0);
                    let borrowed_edge = edge.borrow();

                    if let Some(vertex) = borrowed_edge.destination.clone() {
                        stack.push(vertex);
                    }

                    if visited_edges
                        .iter()
                        .any(|edge| edge.to_string() == borrowed_edge.identifier)
                    {
                        return None;
                    }

                    visited_edges.push(borrowed_edge.identifier);
                } else {
                    let length = vertex.edges.len();
                    let random_choice: usize = rand::thread_rng().gen_range(0, length - 1);
                    let edge = vertex.edges.index(random_choice);
                    let borrowed_edge = edge.borrow();

                    if let Some(vertex) = borrowed_edge.destination.clone() {
                        stack.push(vertex);
                    }

                    if visited_edges
                        .iter()
                        .any(|edge| edge.to_string() == borrowed_edge.identifier)
                    {
                        return None;
                    }

                    visited_edges.push(borrowed_edge.identifier);
                }
            } else {
                return None;
            }
        }

        Some(visited_vertices)
    }

    fn is_fully_fit<'input>(
        vector: &Vec<Rc<RefCell<Vertex>>>,
        source: &'input str,
        destination: &'input str,
    ) -> bool {
        if vector.len() == 0 || vector.len() == 1 {
            return false;
        }

        let first = vector.first().unwrap();
        let last = vector.last().unwrap();

        first.borrow().name == source && last.borrow().name == destination
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
        graph.connect_vertices("Polevskoy", "Ekaterinburg", "poek", 5);
        graph.connect_vertices("Ekaterinburg", "Revda", "ekre", 3);
        graph.connect_vertices("Sysert", "Polevskoy", "sypo", 3);
        graph.connect_vertices("Ekaterinburg", "Sysert", "eksy", 3);

        let random_paths = graph.generate_random_routes("Polevskoy", "Pervouralsk", 10000);

        let fully_fit_paths = random_paths
            .iter()
            .filter(|path| Graph::is_fully_fit(path, "Polevskoy", "Pervouralsk"))
            .count();

        assert!(fully_fit_paths > 0);
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
