use crate::internal::mutable_vertex_pair::MutableVertexPair;
use crate::internal::types::MutableVertexReferences;
use crate::nodes::vertex::Vertex;
use rand::{Rng, thread_rng};
use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct ProcessingAlgorithm;

impl ProcessingAlgorithm {
    pub(crate) fn generate_random_routes(
        &self,
        vertices: &MutableVertexReferences,
        source: &str,
        destination: &str,
        amount_of_generations: i32,
    ) -> Vec<MutableVertexReferences> {
        let starting_point = vertices.iter().find(|v| {
            let borrowed_v = v.borrow();
            borrowed_v.name == source
        });

        if starting_point.is_none() {
            panic!("Create vertex first");
        }

        let mut random_paths = vec![];

        for _ in 0..amount_of_generations {
            if let Some(path) = self.generate_random_route(starting_point.unwrap(), destination) {
                random_paths.push(path)
            }
        }

        random_paths
    }

    pub(crate) fn generate_pairs(
        &self,
        vertices: &Vec<MutableVertexReferences>,
    ) -> Vec<MutableVertexPair> {
        let mut return_vertices = vec![];
        let mut left: Option<MutableVertexReferences> = None;
        let mut right: Option<MutableVertexReferences> = None;

        vertices.iter().for_each(|vertex| {
            if left.is_none() {
                left = Some(vertex.clone());
            } else if right.is_none() {
                right = Some(vertex.clone());
            }

            if left.is_some() && right.is_some() {
                let new_pair = MutableVertexPair {
                    left: left.clone().unwrap(),
                    right: right.clone().unwrap(),
                };

                return_vertices.push(new_pair);

                left = None;
                right = None;
            }
        });

        return_vertices
    }

    pub(crate) fn crossover(&self, pairs: Vec<MutableVertexPair>) -> Vec<MutableVertexReferences> {
        let mut crossed: Vec<MutableVertexReferences> = vec![];

        for pair in pairs {
            let mut same_points = vec![];

            // Selecting same route points with right in left pair
            pair.left.iter().for_each(|vertex| {
                let borrowed_v = vertex.borrow();

                let found_in_right_vector = pair
                    .right
                    .iter()
                    .map(|v| v.borrow())
                    .any(|v| v.name == borrowed_v.name);
                if found_in_right_vector {
                    same_points.push(vertex.clone());
                }
            });

            // Select random index of found same pairs
            let random_index = thread_rng().gen_range(0, same_points.len() - 1);
            let random_vertex = same_points.index(random_index);

            // Searching indexes of same route in right and left vertices
            let mut left_index = 0;
            _ = pair.left.iter().find(|vertex| {
                let borrowed_v = vertex.borrow();
                let right_vertex = random_vertex.borrow();
                left_index += 1;
                borrowed_v.name == right_vertex.name
            });

            let mut right_index = 0;
            _ = pair.right.iter().any(|vertex| {
                right_index += 1;
                let right_vertex = vertex.borrow();
                let random_vertex_borrowed = random_vertex.borrow();
                right_vertex.name == random_vertex_borrowed.name
            });

            let right_slice = pair.right[..right_index].to_vec();
            let left_slice = pair.left[left_index..].to_vec();
            let mut index_to_remove_edge_in_right = 0;
            let mut index_to_remove_edge_in_left = 0;
            let mut weight = 0;

            // Searching index of edges in left, right slice to remove
            let right_last_vertex = right_slice.last().unwrap();
            let left_first_vertex = left_slice.first().unwrap();
            {
                let borrowed_last = right_last_vertex.borrow();
                let borrowed_first = left_first_vertex.borrow();

                _ = borrowed_last.edges.iter().map(|e| e.borrow()).any(|edge| {
                    index_to_remove_edge_in_right += 1;
                    let destination_vertex = edge.destination.as_ref();
                    match destination_vertex {
                        None => false,
                        Some(v) => {
                            let borrowed = v.borrow();
                            let equal = borrowed.name == borrowed_first.name;
                            if equal {
                                weight = edge.weight;
                            }
                            return equal;
                        }
                    }
                });

                _ = borrowed_first.edges.iter().map(|e| e.borrow()).any(|edge| {
                    index_to_remove_edge_in_left += 1;
                    let source_vertex = edge.source.as_ref();
                    match source_vertex {
                        None => false,
                        Some(v) => {
                            let borrowed = v.borrow();
                            borrowed.name == borrowed_last.name
                        }
                    }
                });
            }
            // Removing redundant edges
            {
                let mut borrowed_last = right_last_vertex.borrow_mut();
                let mut borrowed_first = left_first_vertex.borrow_mut();

                borrowed_last.edges.remove(index_to_remove_edge_in_right);
                borrowed_first.edges.remove(index_to_remove_edge_in_left);
            }
            
            // Adding new edge between two slices
            Vertex::add_connection(right_last_vertex, left_first_vertex, weight);

            let mut new_vector = vec![];
            right_slice.iter().for_each(|vertex| {
                new_vector.push(vertex.clone());
            });
            left_slice.iter().for_each(|vertex| {
                new_vector.push(vertex.clone());
            });

            crossed.push(new_vector);
            // TODO: Add tests to check logic
        }

        crossed
    }

    fn generate_random_route(
        &self,
        starting_point: &Rc<RefCell<Vertex>>,
        destination_identity: &str,
    ) -> Option<MutableVertexReferences> {
        let mut stack = vec![];
        let mut resulting_vertices = vec![];
        let mut visited_vertices: Vec<String> = vec![];
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

                    visited_vertices.push(vertex.name.clone());
                } else {
                    let length = vertex.edges.len();
                    let random_choice: usize = thread_rng().gen_range(0, length - 1);
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

                    visited_vertices.push(vertex.name.clone());
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

    fn create_graph() -> Graph {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("Polevskoy".to_owned());
        graph.add_vertex("Revda".to_owned());
        graph.add_vertex("Pervouralsk".to_owned());
        graph.add_vertex("Ekaterinburg".to_owned());
        graph.add_vertex("Sysert".to_owned());

        graph.connect_vertices(
            "Polevskoy".to_owned(),
            "Revda".to_owned(),
            2,
        );
        graph.connect_vertices(
            "Revda".to_owned(),
            "Pervouralsk".to_owned(),
            2,
        );
        graph.connect_vertices(
            "Pervouralsk".to_owned(),
            "Ekaterinburg".to_owned(),
            4,
        );
        graph.connect_vertices(
            "Polevskoy".to_owned(),
            "Ekaterinburg".to_owned(),
            5,
        );
        graph.connect_vertices(
            "Ekaterinburg".to_owned(),
            "Revda".to_owned(),
            3,
        );
        graph.connect_vertices(
            "Sysert".to_owned(),
            "Polevskoy".to_owned(),
            3,
        );
        graph.connect_vertices(
            "Ekaterinburg".to_owned(),
            "Sysert".to_owned(),
            3,
        );

        graph
    }

    #[test]
    pub fn generate_random_paths_should_correctly_return_result() {
        let graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph.vertex_references,
            "Polevskoy",
            "Pervouralsk",
            10,
        );
        let fully_fit_paths = random_routes.iter().count();
        assert!(fully_fit_paths > 0);
    }

    #[test]
    pub fn get_pairs_of_random_routes() {
        let graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph.vertex_references,
            "Polevskoy",
            "Pervouralsk",
            10,
        );

        let result = processing_algorithm.generate_pairs(&random_routes);
        assert!(result.len() > 0);
    }

    #[test]
    pub fn get_crossover_should_return_correct_results() {
        let graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph.vertex_references,
            "Polevskoy",
            "Pervouralsk",
            10,
        );

        let result = processing_algorithm.generate_pairs(&random_routes);
        let crossed = processing_algorithm.crossover(result);
        assert!(crossed.len() > 0);
    }
}
