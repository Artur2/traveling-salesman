use crate::nodes::vertex::Vertex;
use crate::types::MutableVertexReferences;
use rand::Rng;
use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

#[derive(Default)]
pub struct ProcessingAlgorithm;

impl ProcessingAlgorithm {
    pub fn generate_random_routes<'a>(
        &self,
        vertices: &MutableVertexReferences<'a>,
        source: &str,
        destination: &str,
        amount_of_generations: i32,
    ) -> Vec<MutableVertexReferences<'a>> {
        let starting_point = vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == source
        });

        if starting_point.is_none() {
            panic!("Create vertex first");
        }

        let mut random_paths = vec![];

        for _ in 0..amount_of_generations {
            if let Some(path) =
                self.generate_random_route(starting_point.unwrap().clone(), destination)
            {
                random_paths.push(path)
            }
        }

        random_paths
    }

    fn generate_random_route<'b>(
        &self,
        starting_point: Rc<RefCell<Vertex<'b>>>,
        destination_identity: &str,
    ) -> Option<MutableVertexReferences<'b>> {
        let mut stack = vec![];
        let mut resulting_vertices = vec![];
        let mut visited_vertices: Vec<&str> = vec![];
        let starting_rc = starting_point;

        stack.push(starting_rc.clone());

        while !stack.is_empty() {
            if let Some(current) = stack.pop() {
                let vertex = current.borrow();
                resulting_vertices.push(current.clone());

                if vertex.name == destination_identity {
                    return Some(resulting_vertices);
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

                    if visited_vertices
                        .iter()
                        .any(|vertex_name| vertex_name == &vertex.name)
                    {
                        return None;
                    }

                    visited_vertices.push(vertex.name);
                } else {
                    let length = vertex.edges.len();
                    let random_choice: usize = rand::thread_rng().gen_range(0, length - 1);
                    let edge = vertex.edges.index(random_choice);
                    let borrowed_edge = edge.borrow();

                    if let Some(vertex) = borrowed_edge.destination.clone() {
                        stack.push(vertex);
                    }

                    if visited_vertices
                        .iter()
                        .any(|vertex_name| vertex_name == &vertex.name)
                    {
                        return None;
                    }

                    visited_vertices.push(vertex.name);
                }
            } else {
                return None;
            }
        }

        Some(resulting_vertices)
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
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

        let processing_algorithm = ProcessingAlgorithm::default();
        let random_paths = processing_algorithm.generate_random_routes(
            &graph.vertex_references,
            "Polevskoy",
            "Pervouralsk",
            10,
        );

        let fully_fit_paths = random_paths.iter().count();

        assert!(fully_fit_paths > 0);
    }
}
