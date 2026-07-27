use crate::internal::mutable_vertex_pair::MutableVertexPair;
use crate::internal::types::{
    MutableVertexReference, MutableVertexReferences, WeakVertexReference, WeakVertexReferences,
};
use crate::nodes::graph::Graph;
use crate::nodes::vertex::Vertex;
use rand::{Rng, thread_rng};
use std::cell::RefCell;
use std::cmp;
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
        amount_of_generations: u32,
    ) -> Vec<WeakVertexReferences> {
        // TODO: Rewrite collecting edges instead of vertices
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
                let first = path.first().unwrap();
                let last = path.last().unwrap();

                let borrowed_first = first.upgrade();
                let borrowed_last = last.upgrade();
                if let (Some(first), Some(last)) = (borrowed_first, borrowed_last) {
                    let borrowed_source = first.borrow();
                    let borrowed_destination = last.borrow();
                    if borrowed_source.name == source && borrowed_destination.name == destination {
                        random_paths.push(path)
                    }
                }
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
        for pair in pairs {
            let mut same_points = vec![];

            // Selecting same route points with right in left pair
            self.select_same_route_points_in_pair(&pair, &mut same_points);

            // Select random index of found same pairs
            let random_index = thread_rng().gen_range(0, same_points.len() - 1);
            let random_vertex = same_points.index(random_index);

            // Searching indexes of same route in right and left vertices
            let (left_index, right_index) =
                self.select_same_point_in_left_right_routes(&pair, &random_vertex);

            let right_slice = pair.right[..right_index as usize].to_vec();
            let left_slice = pair.left[left_index as usize..].to_vec();

            // Calculating edges to remove
            let (right_index, left_index, right_last_vertex, left_first_vertex, weight) =
                self.calculate_edges_in_left_right_routes_to_remove(&right_slice, &left_slice);

            // Removing redundant edges
            self.remove_edges(
                right_last_vertex,
                left_first_vertex,
                right_index,
                left_index,
            );

            let new_vector = self.add_new_edges(
                graph,
                right_last_vertex,
                left_first_vertex,
                weight,
                &right_slice,
                &left_slice,
            );
            crossed.push(new_vector);
            // TODO: Add tests to check logic
        }

        crossed
    }

    pub(crate) fn get_fit_value(&self, route: &WeakVertexReferences) -> u32 {
        let route_path = route
            .iter()
            .filter_map(|f| f.upgrade())
            .map(|vertex| vertex.borrow().name.clone())
            .collect::<Vec<String>>();

        let mut fit = 0;
        route.iter().enumerate().for_each(|(index, vertex)| {
            if index + 1 >= route_path.len() {
                return;
            }
            let current_vertex_name = route_path[index].clone();
            let next_vertex_name = route_path[index + 1].clone();

            match vertex.upgrade() {
                None => panic!("Cant reach out vertex"),
                Some(vertex_upgraded) => {
                    let borrowed = vertex_upgraded.borrow();

                    let edge = borrowed
                        .edges
                        .iter()
                        .map(|p| p.upgrade())
                        .map(|p| p.unwrap())
                        .find(|p| {
                            let edge = p.borrow();

                            if let (Some(source_unwrapped), Some(destination_unwrapped)) =
                                (edge.source.as_ref(), edge.destination.as_ref())
                                && let (Some(source), Some(destination)) =
                                    (source_unwrapped.upgrade(), destination_unwrapped.upgrade())
                            {
                                let source_borrowed = source.borrow();
                                let destination_borrowed = destination.borrow();

                                return source_borrowed.name == current_vertex_name
                                    && destination_borrowed.name == next_vertex_name;
                            }

                            false
                        });

                    if let Some(edge) = edge {
                        let borrowed = edge.borrow();
                        fit += borrowed.weight;
                    } else {
                        panic!("Cant reach out edge");
                    }
                }
            }
        });

        fit
    }

    fn add_new_edges(
        &self,
        graph: &mut Graph,
        right_last_vertex: &WeakVertexReference,
        left_first_vertex: &WeakVertexReference,
        weight: u32,
        right: &WeakVertexReferences,
        left: &WeakVertexReferences,
    ) -> Vec<WeakVertexReference> {
        // Adding new edge between two slices
        Vertex::add_connection(graph, right_last_vertex, left_first_vertex, weight);

        let mut new_vector = vec![];
        right.iter().for_each(|vertex| {
            new_vector.push(vertex.clone());
        });
        left.iter().for_each(|vertex| {
            new_vector.push(vertex.clone());
        });

        new_vector
    }

    fn remove_edges(
        &self,
        right: &WeakVertexReference,
        left: &WeakVertexReference,
        right_index: u32,
        left_index: u32,
    ) {
        let borrowed_last_upgraded = right.upgrade();
        let borrowed_first_upgraded = left.upgrade();

        if let (Some(borrowed_last), Some(borrowed_first)) =
            (borrowed_last_upgraded, borrowed_first_upgraded)
        {
            let mut borrowed_last = borrowed_last.borrow_mut();
            let mut borrowed_first = borrowed_first.borrow_mut();

            borrowed_last.edges.remove(right_index as usize);
            borrowed_first.edges.remove(left_index as usize);
        } else {
            panic!("Cant reach out vertex");
        }
    }

    fn select_same_route_points_in_pair(
        &self,
        pair: &MutableVertexPair,
        vertices: &mut MutableVertexReferences,
    ) {
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
                    vertices.push(vertex.clone());
                }
            });
    }

    fn select_same_point_in_left_right_routes(
        &self,
        pair: &MutableVertexPair,
        vertex: &MutableVertexReference,
    ) -> (i32, i32) {
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
                let right_vertex = vertex.borrow();
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
                let random_vertex_borrowed = vertex.borrow();
                right_vertex.name == random_vertex_borrowed.name
            });

        (left_index, right_index)
    }

    fn calculate_edges_in_left_right_routes_to_remove<'a>(
        &self,
        right: &'a WeakVertexReferences,
        left: &'a WeakVertexReferences,
    ) -> (
        u32,
        u32,
        &'a WeakVertexReference,
        &'a WeakVertexReference,
        u32,
    ) {
        // Searching index of edges in left, right slice to remove
        let mut index_to_remove_edge_in_right: i32 = -1;
        let mut index_to_remove_edge_in_left: i32 = -1;
        let mut weight = 0;
        let right_last_vertex = right.last().unwrap();
        let left_first_vertex = left.first().unwrap();
        let borrowed_last_upgrade = right_last_vertex.upgrade();
        if borrowed_last_upgrade.is_none() {
            panic!("Cant reach out vertex");
        }
        let borrowed_first_updgrade = left_first_vertex.upgrade();
        if borrowed_first_updgrade.is_none() {
            panic!("Cant reach out vertex");
        }

        if let (Some(borrowed_first), Some(borrowed_last)) =
            (borrowed_first_updgrade, borrowed_last_upgrade)
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
                                equal
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

        (
            index_to_remove_edge_in_right as u32,
            index_to_remove_edge_in_left as u32,
            right_last_vertex,
            left_first_vertex,
            weight,
        )
    }

    fn generate_random_route(
        &self,
        starting_point: &Weak<RefCell<Vertex>>,
        destination_identity: &str,
    ) -> Option<WeakVertexReferences> {
        // TODO: Simplify
        let mut stack = vec![];
        let mut resulting_vertices = vec![];
        let mut visited_routes: Vec<String> = vec![];
        let starting_rc = starting_point;

        stack.push(starting_rc.clone());

        while !stack.is_empty() {
            if let Some(current) = stack.pop() {
                match current.upgrade() {
                    None => panic!("Cant reach out vertex"),
                    Some(current) => {
                        let vertex = current.borrow();
                        if !resulting_vertices.iter().any(|v: &WeakVertexReference| {
                            match v.upgrade() {
                                Some(vertex) => {
                                    let ptr = vertex.as_ptr();
                                    let current = current.as_ptr();
                                    std::ptr::eq(ptr, current)
                                }
                                None => panic!("Cant reach out vertex"),
                            }
                        }) {
                            resulting_vertices.push(Rc::downgrade(&current));
                        }

                        if vertex.name == destination_identity {
                            return Some(resulting_vertices);
                        }

                        if vertex.edges.is_empty() {
                            // vertex disconnected?
                            continue;
                        }

                        let except_edges = &vertex.edges;
                        if except_edges.is_empty() {
                            continue;
                        }

                        let length = except_edges.len();
                        if length == 0 {
                            continue;
                        }
                        let max = cmp::max(length - 1, 0);
                        let random_choice: usize = if max > 0 {
                            thread_rng().gen_range(0, max)
                        } else {
                            0
                        };
                        let edge = except_edges.index(random_choice);
                        match edge.upgrade() {
                            None => panic!("Cant reach out edge"),
                            Some(edge) => {
                                let borrowed_edge = edge.borrow();

                                if let (Some(vertex_destination), Some(vertex_source)) = (
                                    borrowed_edge.destination.clone(),
                                    borrowed_edge.source.clone(),
                                ) {
                                    if !visited_routes
                                        .iter()
                                        .any(|v| *v == borrowed_edge.identifier)
                                    {
                                        stack.push(vertex_source.clone());
                                        stack.push(vertex_destination.clone());
                                        visited_routes.push(borrowed_edge.identifier.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                continue;
            }
        }

        Some(resulting_vertices)
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::nodes::graph::Graph;
    use std::time::Instant;

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

    fn create_complex_graph() -> Graph {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("Верхняя Пышма".to_owned());
        graph.add_vertex("Березовский".to_owned());
        graph.add_vertex("Арамиль".to_owned());
        graph.add_vertex("Сысерть".to_owned());
        graph.add_vertex("Полевской".to_owned());
        graph.add_vertex("Заречный".to_owned());
        graph.add_vertex("Дегтярск".to_owned());
        graph.add_vertex("Ревда".to_owned());
        graph.add_vertex("Асбест".to_owned());
        graph.add_vertex("Белоярский".to_owned());
        graph.add_vertex("Первоуральск".to_owned());
        graph.add_vertex("Екатеринбург".to_owned());

        graph.connect_vertices("Верхняя Пышма".to_owned(), "Екатеринбург".to_owned(), 17);
        graph.connect_vertices("Березовский".to_owned(), "Екатеринбург".to_owned(), 14);
        graph.connect_vertices("Екатеринбург".to_owned(), "Дегтярск".to_owned(), 71);
        graph.connect_vertices("Екатеринбург".to_owned(), "Арамиль".to_owned(), 30);
        graph.connect_vertices("Арамиль".to_owned(), "Белоярский".to_owned(), 47);
        graph.connect_vertices("Екатеринбург".to_owned(), "Заречный".to_owned(), 61);
        graph.connect_vertices("Екатеринбург".to_owned(), "Полевской".to_owned(), 68);
        graph.connect_vertices("Екатеринбург".to_owned(), "Ревда".to_owned(), 54);
        graph.connect_vertices("Екатеринбург".to_owned(), "Асбест".to_owned(), 90);
        graph.connect_vertices("Екатеринбург".to_owned(), "Белоярский".to_owned(), 53);
        graph.connect_vertices("Верхняя Пышма".to_owned(), "Березовский".to_owned(), 23);
        graph.connect_vertices("Заречный".to_owned(), "Березовский".to_owned(), 57);
        graph.connect_vertices("Заречный".to_owned(), "Белоярский".to_owned(), 11);
        graph.connect_vertices("Асбест".to_owned(), "Заречный".to_owned(), 42);
        graph.connect_vertices("Ревда".to_owned(), "Дегтярск".to_owned(), 21);
        graph.connect_vertices("Дегтярск".to_owned(), "Полевской".to_owned(), 38);
        graph.connect_vertices("Верхняя Пышма".to_owned(), "Первоуральск".to_owned(), 59);
        graph.connect_vertices("Первоуральск".to_owned(), "Ревда".to_owned(), 15);
        graph.connect_vertices("Сысерть".to_owned(), "Арамиль".to_owned(), 28);

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

    #[test]
    pub fn get_fit_value_should_correct_pass_through() {
        let graph = &mut create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let mut time = Instant::now();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            "Polevskoy",
            "Pervouralsk",
            100,
        );

        let elapsed_generation = time.elapsed().as_micros();
        time = Instant::now();
        random_routes.iter().for_each(|r| {
            let fit_value = processing_algorithm.get_fit_value(r);
            assert!(fit_value > 0);
        });

        let elapsed_fit = time.elapsed().as_micros();
        println!(
            "Generated in {:?} μs, fitted in {:?} μs",
            elapsed_generation, elapsed_fit
        );
    }

    #[test]
    pub fn crossover_should_work_as_expected() {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("A".to_owned());
        graph.add_vertex("B".to_owned());
        graph.add_vertex("C".to_owned());
        graph.add_vertex("D".to_owned());
        graph.add_vertex("E".to_owned());

        graph.connect_vertices("A".to_owned(), "B".to_owned(), 2);
        graph.connect_vertices("B".to_owned(), "C".to_owned(), 2);
        graph.connect_vertices("C".to_owned(), "D".to_owned(), 2);
        graph.connect_vertices("D".to_owned(), "E".to_owned(), 2);
        graph.connect_vertices("B".to_owned(), "D".to_owned(), 4);
        graph.connect_vertices("B".to_owned(), "E".to_owned(), 4);
        graph.connect_vertices("C".to_owned(), "E".to_owned(), 4);

        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            "A",
            "E",
            100,
        );
        let pairs = processing_algorithm.generate_pairs(&random_routes);
        let result = processing_algorithm.crossover(&mut graph, pairs);

        assert!(result.len() > 0);
    }

    #[test]
    pub fn should_find_optimal_way_for_complex_graph() {
        let mut graph = create_complex_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let mut min_value = i32::MAX;
        while min_value != 3 {
            let random_routes = processing_algorithm.generate_random_routes(
                &graph
                    .vertex_references
                    .iter()
                    .map(|f| Rc::downgrade(&f))
                    .collect(),
                "Ревда",
                "Белоярский",
                10,
            );

            random_routes.iter().for_each(|r| {
                let len = r.len();
                if len < min_value as usize {
                    min_value = len as i32;
                }
            })
        }
    }
}
