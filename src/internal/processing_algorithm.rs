use crate::graph::Graph;
use crate::internal::mutable_vertex_pair::MutableVertexPair;
use crate::internal::types::WeakVertexReferences;
use crate::nodes::vertex::Vertex;
use rand::{Rng, thread_rng};
use std::cell::RefCell;
use std::ops::Index;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct ProcessingAlgorithm;

impl ProcessingAlgorithm {
    pub(crate) fn generate_random_routes(
        &self,
        vertices: &WeakVertexReferences,
        source: &str,
        destination: &str,
        amount_of_generations: i32,
    ) -> Vec<WeakVertexReferences> {
        let starting_point = vertices.iter().find(|v| match v.upgrade() {
            None => false,
            Some(v) => {
                let borrowed_v = v.borrow();
                borrowed_v.name == source
            }
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
        vertices: &Vec<WeakVertexReferences>,
    ) -> Vec<MutableVertexPair> {
        let mut return_vertices = vec![];
        let mut left: Option<WeakVertexReferences> = None;
        let mut right: Option<WeakVertexReferences> = None;

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

    pub(crate) fn crossover(
        &self,
        graph: &mut Graph,
        pairs: Vec<MutableVertexPair>,
    ) -> Vec<WeakVertexReferences> {
        let mut crossed: Vec<WeakVertexReferences> = vec![];
        // TODO: Simplify
        for pair in pairs {
            let mut same_points = vec![];

            // Selecting same route points with right in left pair
            pair.left
                .iter()
                .map(|v| match v.upgrade() {
                    None => panic!("Cant reach out vertex"),
                    Some(vertex) => vertex,
                })
                .for_each(|vertex| {
                    let borrowed_v = vertex.borrow();
                    let found_in_right_vector = pair
                        .right
                        .iter()
                        .map(|v| match v.upgrade() {
                            None => panic!("Cant reach out vertex"),
                            Some(vertex) => vertex,
                        })
                        .any(|v| match v.try_borrow() {
                            Ok(v) => v.name == borrowed_v.name,
                            Err(_) => false,
                        });
                    if found_in_right_vector {
                        same_points.push(vertex.clone());
                    }
                });

            // Select random index of found same pairs
            let random_index = thread_rng().gen_range(0, same_points.len() - 1);
            let random_vertex = same_points.index(random_index);

            // Searching indexes of same route in right and left vertices
            let mut left_index = 0;
            _ = pair
                .left
                .iter()
                .map(|v| match v.upgrade() {
                    None => panic!("Cant reach out vertex"),
                    Some(vertex) => vertex,
                })
                .any(|vertex| {
                    left_index += 1;
                    let borrowed_v = vertex.borrow();
                    let right_vertex = random_vertex.borrow();
                    borrowed_v.name == right_vertex.name
                });

            let mut right_index = 0;
            _ = pair
                .right
                .iter()
                .map(|v| match v.upgrade() {
                    None => panic!("Cant reach out vertex"),
                    Some(vertex) => vertex,
                })
                .any(|vertex| {
                    right_index += 1;
                    let right_vertex = vertex.borrow();
                    let random_vertex_borrowed = random_vertex.borrow();
                    right_vertex.name == random_vertex_borrowed.name
                });

            let right_slice = pair.right[..right_index].to_vec();
            let left_slice = pair.left[left_index..].to_vec();
            let mut index_to_remove_edge_in_right: i32 = -1;
            let mut index_to_remove_edge_in_left: i32 = -1;
            let mut weight = 0;

            // Searching index of edges in left, right slice to remove
            let right_last_vertex = right_slice.last().unwrap();
            let left_first_vertex = left_slice.first().unwrap();
            {
                let borrowed_last_updgrade = right_last_vertex.upgrade();
                if borrowed_last_updgrade.is_none() {
                    panic!("Cant reach out vertex");
                }
                let borrowed_first_updgrade = left_first_vertex.upgrade();
                if borrowed_first_updgrade.is_none() {
                    panic!("Cant reach out vertex");
                }

                if let (Some(borrowed_first), Some(borrowed_last)) =
                    (borrowed_first_updgrade, borrowed_last_updgrade)
                {
                    _ = borrowed_last
                        .borrow()
                        .edges
                        .iter()
                        .map(|e| match e.upgrade() {
                            None => {
                                panic!("Cant reach out edge");
                            }
                            Some(edge) => edge,
                        })
                        .any(|edge| {
                            index_to_remove_edge_in_right += 1;
                            let borrowed_edge = edge.borrow();
                            let destination_vertex = borrowed_edge.destination.as_ref();
                            match destination_vertex {
                                None => false,
                                Some(v) => match v.upgrade() {
                                    None => panic!("Cant reach out vertex"),
                                    Some(v) => {
                                        let borrowed = v.borrow();
                                        let equal = borrowed.name == borrowed_first.borrow().name;
                                        if equal {
                                            weight = borrowed_edge.weight;
                                        }
                                        return equal;
                                    }
                                },
                            }
                        });

                    _ = borrowed_first
                        .borrow()
                        .edges
                        .iter()
                        .map(|e| match e.upgrade() {
                            None => {
                                panic!("Cant reach out edge");
                            }
                            Some(edge) => edge,
                        })
                        .any(|edge| {
                            index_to_remove_edge_in_left += 1;
                            let borrowed_edge = edge.borrow();
                            let source_vertex = borrowed_edge.source.as_ref();
                            match source_vertex {
                                None => false,
                                Some(v) => match v.upgrade() {
                                    None => panic!("Cant reach out vertex"),
                                    Some(v) => {
                                        let borrowed = v.borrow();
                                        borrowed.name == borrowed_last.borrow().name
                                    }
                                },
                            }
                        });
                }
            }
            // Removing redundant edges
            {
                let borrowed_last_upgraded = right_last_vertex.upgrade();
                let borrowed_first_upgraded = left_first_vertex.upgrade();

                if let (Some(borrowed_last), Some(borrowed_first)) =
                    (borrowed_last_upgraded, borrowed_first_upgraded)
                {
                    let mut borrowed_last = borrowed_last.borrow_mut();
                    let mut borrowed_first = borrowed_first.borrow_mut();

                    borrowed_last
                        .edges
                        .remove(index_to_remove_edge_in_right as usize);
                    borrowed_first
                        .edges
                        .remove(index_to_remove_edge_in_left as usize);
                } else {
                    panic!("Cant reach out vertex");
                }
            }

            // Adding new edge between two slices
            Vertex::add_connection(graph, right_last_vertex, left_first_vertex, weight);

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
        starting_point: &Weak<RefCell<Vertex>>,
        destination_identity: &str,
    ) -> Option<WeakVertexReferences> {
        let mut stack = vec![];
        let mut resulting_vertices = vec![];
        let mut visited_vertices: Vec<String> = vec![];
        let starting_rc = starting_point;

        stack.push(starting_rc.clone());

        while !stack.is_empty() {
            if let Some(current) = stack.pop() {
                match current.upgrade() {
                    None => panic!("Cant reach out vertex"),
                    Some(current) => {
                        let vertex = current.borrow();
                        resulting_vertices.push(Rc::downgrade(&current));

                        if vertex.name == destination_identity {
                            return Some(resulting_vertices);
                        }

                        if vertex.edges.len() == 0 {
                            // vertex disconnected?
                            return None;
                        }

                        if vertex.edges.len() == 1 {
                            let edge = vertex.edges.index(0);
                            match edge.upgrade() {
                                None => panic!("Cant reach out edge"),
                                Some(edge) => {
                                    let borrowed_edge = edge.borrow();

                                    if let Some(vertex) = borrowed_edge.destination.clone() {
                                        stack.push(vertex);
                                    }
                                }
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
                            match edge.upgrade() {
                                None => panic!("Cant reach out edge"),
                                Some(edge) => {
                                    let borrowed_edge = edge.borrow();

                                    if let Some(vertex) = borrowed_edge.destination.clone() {
                                        stack.push(vertex);
                                    }
                                }
                            }

                            if visited_vertices
                                .iter()
                                .any(|vertex_name| vertex_name == &vertex.name)
                            {
                                return None;
                            }

                            visited_vertices.push(vertex.name.clone());
                        }
                    }
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

        graph.connect_vertices("Polevskoy".to_owned(), "Revda".to_owned(), 2);
        graph.connect_vertices("Revda".to_owned(), "Pervouralsk".to_owned(), 2);
        graph.connect_vertices("Pervouralsk".to_owned(), "Ekaterinburg".to_owned(), 4);
        graph.connect_vertices("Polevskoy".to_owned(), "Ekaterinburg".to_owned(), 5);
        graph.connect_vertices("Ekaterinburg".to_owned(), "Revda".to_owned(), 3);
        graph.connect_vertices("Sysert".to_owned(), "Polevskoy".to_owned(), 3);
        graph.connect_vertices("Ekaterinburg".to_owned(), "Sysert".to_owned(), 3);

        graph
    }

    #[test]
    pub fn generate_random_paths_should_correctly_return_result() {
        let graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
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
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            "Polevskoy",
            "Pervouralsk",
            10,
        );

        let result = processing_algorithm.generate_pairs(&random_routes);
        assert!(result.len() > 0);
    }

    #[test]
    pub fn get_crossover_should_return_correct_results() {
        let graph = &mut create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            "Polevskoy",
            "Pervouralsk",
            1000,
        );

        let result = processing_algorithm.generate_pairs(&random_routes);
        let crossed = processing_algorithm.crossover(graph, result);
        assert!(crossed.len() > 0);
    }
}
