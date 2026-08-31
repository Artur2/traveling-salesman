use crate::internal::mutable_vertex_pair::MutableVertexPair;
use crate::internal::string_pool::StringPool;
use crate::internal::types::{
    MutableVertexReference, MutableVertexReferences, WeakVertexReferences,
};
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use crate::{random_index, upgrade_conditionally};
use rand::{Rng, thread_rng};
use std::cell::RefCell;
use std::cmp::max;
use std::ops::Index;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct ProcessingAlgorithm;

impl ProcessingAlgorithm {
    pub(crate) fn generate_random_routes(
        &self,
        string_pool: &mut StringPool,
        vertices: &WeakVertexReferences,
        source: &String,
        destination: &String,
        amount_of_generations: u32,
    ) -> Vec<MutableVertexReferences> {
        let starting_point = vertices.iter().find(|v| {
            let value = upgrade_conditionally!(v);
            let borrowed_v = value.borrow();
            let name = upgrade_conditionally!(&borrowed_v.name);
            name.as_ref() == source
        });

        if starting_point.is_none() {
            panic!("Create vertex first");
        }

        let mut random_paths = vec![];
        let mut passed_cycles = amount_of_generations;
        while passed_cycles > 0 {
            let routes =
                self.generate_random_route(string_pool, starting_point.unwrap(), destination);
            passed_cycles -= 1;
            if routes.is_empty() {
                // Пропускаем роуты, которые не дошли до конечной точки
                continue;
            }

            random_paths.push(routes);
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

    pub(crate) fn crossover(
        &self,
        string_pool: &mut StringPool,
        pairs: &Vec<MutableVertexPair>,
    ) -> Vec<MutableVertexReferences> {
        let mut crossed: Vec<MutableVertexReferences> = vec![];
        for pair in pairs {
            // Selecting same route points with right in left pair
            let same_points = self.select_same_route_points_in_pair(&pair);

            // Select random index of found same pairs
            let random_index = thread_rng().gen_range(0, same_points.len() - 1);
            let random_vertex = same_points.index(random_index);

            // Searching indexes of same route in right and left vertices
            let (left_index, right_index) =
                self.select_same_point_in_left_right_routes(pair, random_vertex);

            let right_slice = pair.right[..right_index as usize].to_vec();
            let left_slice = pair.left[left_index as usize..].to_vec();

            let (right_last_vertex, left_first_vertex, weight) =
                self.calculate_edges_in_left_right_routes_to_add(&right_slice, &left_slice);

            let new_vector = self.add_new_edges(
                string_pool,
                right_last_vertex,
                left_first_vertex,
                weight,
                &right_slice,
                &left_slice,
            );
            crossed.push(new_vector);
        }

        crossed
    }

    pub(crate) fn get_fit_value(&self, route: &MutableVertexReferences) -> f64 {
        let route_path = route
            .iter()
            .map(|vertex| vertex.borrow().name.clone())
            .collect::<Vec<Weak<str>>>();

        let mut fit = 0f64;
        route.iter().enumerate().for_each(|(index, vertex)| {
            if index + 1 >= route_path.len() {
                return;
            }
            let current_vertex_name = unsafe { route_path.get_unchecked(index) }.clone();
            let next_vertex_name = unsafe { route_path.get_unchecked(index + 1) }.clone();

            let borrowed = vertex.borrow();

            let edge = borrowed.edges.iter().find(|p| {
                let edge = p.borrow();

                if let (Some(source_unwrapped), Some(destination_unwrapped)) =
                    (edge.source.as_ref(), edge.destination.as_ref())
                {
                    let source_borrowed = source_unwrapped.borrow();
                    let destination_borrowed = destination_unwrapped.borrow();
                    let source_name = upgrade_conditionally!(source_borrowed.name);
                    let destination_name = upgrade_conditionally!(destination_borrowed.name);
                    let current_name = upgrade_conditionally!(current_vertex_name);
                    let next_name = upgrade_conditionally!(next_vertex_name);

                    return (source_name.eq(&current_name) && destination_name.eq(&next_name))
                        || (destination_name.eq(&current_name) && source_name.eq(&next_name));
                }

                false
            });

            if let Some(edge) = edge {
                let borrowed = edge.borrow();
                fit += borrowed.weight;
            } else {
                panic!("Cant reach out edge");
            }
        });

        fit
    }

    fn add_new_edges(
        &self,
        string_pool: &mut StringPool,
        right_last_vertex: &MutableVertexReference,
        left_first_vertex: &MutableVertexReference,
        weight: f64,
        right: &MutableVertexReferences,
        left: &MutableVertexReferences,
    ) -> Vec<MutableVertexReference> {
        Vertex::add_connection(string_pool, right_last_vertex, left_first_vertex, weight);

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
        right: &MutableVertexReference,
        left: &MutableVertexReference,
        right_index: u32,
        left_index: u32,
    ) {
        let mut borrowed_last = right.borrow_mut();
        let mut borrowed_first = left.borrow_mut();

        borrowed_last.edges.remove(right_index as usize);
        borrowed_first.edges.remove(left_index as usize);
    }

    fn select_same_route_points_in_pair(
        &self,
        pair: &MutableVertexPair,
    ) -> MutableVertexReferences {
        let mut same_points = vec![];
        pair.left.iter().for_each(|vertex| {
            let borrowed_v = vertex.borrow();
            let found_in_right_vector = pair.right.iter().any(|v| match v.try_borrow() {
                Ok(v) => {
                    let left_name = upgrade_conditionally!(v.name);
                    let right_name = upgrade_conditionally!(borrowed_v.name);
                    left_name.eq(&right_name)
                }
                Err(_) => false,
            });
            if found_in_right_vector {
                same_points.push(vertex.clone());
            }
        });

        same_points
    }

    fn select_same_point_in_left_right_routes(
        &self,
        pair: &MutableVertexPair,
        random_vertex: &MutableVertexReference,
    ) -> (i32, i32) {
        // Searching indexes of same route in right and left vertices
        let mut left_index = 0;
        _ = pair.left.iter().any(|vertex| {
            left_index += 1;
            let left_vertex = vertex.borrow();
            let vertex_borrow = random_vertex.borrow();
            let borrowed_name = upgrade_conditionally!(vertex_borrow.name);
            let left_name = upgrade_conditionally!(left_vertex.name);
            borrowed_name.eq(&left_name)
        });

        let mut right_index = 0;
        _ = pair.right.iter().any(|vertex| {
            right_index += 1;
            let right_vertex = vertex.borrow();
            let random_vertex_borrowed = random_vertex.borrow();
            let borrowed_name = upgrade_conditionally!(random_vertex_borrowed.name);
            let right_vertex_name = upgrade_conditionally!(right_vertex.name);
            borrowed_name.eq(&right_vertex_name)
        });

        (left_index, right_index)
    }

    fn calculate_edges_in_left_right_routes_to_add<'a>(
        &self,
        right: &'a MutableVertexReferences,
        left: &'a MutableVertexReferences,
    ) -> (&'a MutableVertexReference, &'a MutableVertexReference, f64) {
        let mut weight = 0f64;
        let right_last_vertex = right.last().unwrap();
        let left_first_vertex = left.first().unwrap();

        _ = right_last_vertex.borrow().edges.iter().any(|edge| {
            let borrowed_edge = edge.borrow();
            let destination_vertex = borrowed_edge.destination.as_ref();
            match destination_vertex {
                None => false,
                Some(v) => {
                    let borrowed = v.borrow();
                    let first_vertex_borrowed = left_first_vertex.borrow();
                    let first_vertex_name = upgrade_conditionally!(first_vertex_borrowed.name);
                    let name = upgrade_conditionally!(borrowed.name);

                    let equal = first_vertex_name.eq(&name);
                    if equal {
                        weight = borrowed_edge.weight;
                    }
                    equal
                }
            }
        });

        _ = left_first_vertex.borrow().edges.iter().any(|edge| {
            let borrowed_edge = edge.borrow();
            let source_vertex = borrowed_edge.source.as_ref();
            match source_vertex {
                None => false,
                Some(v) => {
                    let borrowed = v.borrow();
                    let right_vertex = right_last_vertex.borrow();
                    let name = upgrade_conditionally!(borrowed.name);
                    let right_name = upgrade_conditionally!(right_vertex.name);

                    name.eq(&right_name)
                }
            }
        });

        (right_last_vertex, left_first_vertex, weight)
    }

    fn generate_random_route(
        &self,
        string_pool: &mut StringPool,
        starting_point: &Weak<RefCell<Vertex>>,
        destination_identity: &str,
    ) -> Vec<MutableVertexReference> {
        let mut stack = vec![];
        let mut resulting_edges = vec![];
        let mut visited_vertices: Vec<MutableVertexReference> = vec![];
        let mut resulting_vertices = vec![];

        let starting_point_rc = upgrade_conditionally!(starting_point);
        let borrowed_starting_point = starting_point_rc.borrow();
        let random_value = random_index!(&borrowed_starting_point);

        let edge = unsafe { borrowed_starting_point.edges.get_unchecked(random_value) };
        let edge_borrowed = edge.borrow();

        let new_vertex = Rc::new(RefCell::new(Vertex::new(
            borrowed_starting_point.name.clone(),
        )));
        resulting_vertices.push((edge_borrowed.weight, new_vertex));
        stack.push(edge.clone());
        visited_vertices.push(starting_point_rc.clone());

        while !stack.is_empty() {
            match stack.pop() {
                None => panic!("Cant reach out edge"),
                Some(current_edge) => {
                    resulting_edges.push(current_edge.clone());
                    let borrowed_edge = current_edge.borrow();

                    if let (Some(vertex_destination), Some(vertex_source)) =
                        (&borrowed_edge.destination, &borrowed_edge.source)
                    {
                        let borrowed_vertex_destination = vertex_destination.borrow();
                        let borrowed_vertex_source = vertex_source.borrow();

                        // Если не посетили данную точку, то все путь кладем в стек
                        if !visited_vertices.iter().any(|f| {
                            let borrowed_vertex = f.borrow();
                            let name_left = upgrade_conditionally!(borrowed_vertex.name);
                            let name_right = upgrade_conditionally!(borrowed_vertex_source.name);
                            name_left.eq(&name_right)
                        }) {
                            let random_value = random_index!(&borrowed_vertex_source);
                            let random_edge =
                                unsafe { borrowed_vertex_source.edges.get_unchecked(random_value) };
                            stack.push(random_edge.clone());
                            visited_vertices.push(vertex_source.clone());

                            let new_vertex = Vertex::new(borrowed_vertex_source.name.clone());
                            let new_vertex_rc = Rc::new(RefCell::new(new_vertex));
                            resulting_vertices.push((borrowed_edge.weight, new_vertex_rc.clone()));

                            let vertex_source_name =
                                upgrade_conditionally!(borrowed_vertex_source.name);
                            if vertex_source_name.as_ref() == destination_identity {
                                return self.connect_vertices(string_pool, resulting_vertices);
                            }
                        }

                        // Если не посетили данную точку, то все пути кладем в стек
                        if !visited_vertices.iter().any(|f| {
                            let borrowed_vertex = f.borrow();
                            let name_left = upgrade_conditionally!(borrowed_vertex.name);
                            let name_right =
                                upgrade_conditionally!(borrowed_vertex_destination.name);
                            name_left.eq(&name_right)
                        }) {
                            let random_value = random_index!(&borrowed_vertex_destination);
                            let random_edge = unsafe {
                                borrowed_vertex_destination
                                    .edges
                                    .get_unchecked(random_value)
                            };
                            stack.push(random_edge.clone());
                            visited_vertices.push(vertex_destination.clone());

                            let new_vertex = Vertex::new(borrowed_vertex_destination.name.clone());
                            let new_vertex_rc = Rc::new(RefCell::new(new_vertex));
                            resulting_vertices.push((borrowed_edge.weight, new_vertex_rc.clone()));
                            let borrowed_vertex_name =
                                upgrade_conditionally!(borrowed_vertex_destination.name);
                            if borrowed_vertex_name.as_ref() == destination_identity {
                                return self.connect_vertices(string_pool, resulting_vertices);
                            }
                        }
                    }
                }
            }
        }

        vec![]
    }

    fn connect_vertices(
        &self,
        string_pool: &mut StringPool,
        resulting_vertices: Vec<(f64, MutableVertexReference)>,
    ) -> Vec<MutableVertexReference> {
        let mut new_vertexes = vec![];
        for (i, result) in resulting_vertices.iter().enumerate() {
            new_vertexes.push(result.1.clone());
            let next_index = i + 1;
            if next_index >= resulting_vertices.len() {
                break;
            }

            let current_vertex = unsafe { resulting_vertices.get_unchecked(i) };
            let next_vertex = unsafe { resulting_vertices.get_unchecked(next_index) };

            let mut borrowed_vertex = current_vertex.1.borrow_mut();
            let mut borrowed_next_vertex = next_vertex.1.borrow_mut();
            let borrowed_vertex_name = upgrade_conditionally!(borrowed_vertex.name);
            let borrowed_next_vertex_name = upgrade_conditionally!(borrowed_next_vertex.name);
            let name = format!("{}-{}", borrowed_vertex_name, borrowed_next_vertex_name);
            let mut new_edge = Edge::new(string_pool.intern(name), current_vertex.0);
            new_edge.source = Some(current_vertex.1.clone());
            new_edge.destination = Some(next_vertex.1.clone());
            let new_edge_rc = Rc::new(RefCell::new(new_edge));

            borrowed_vertex.edges.push(new_edge_rc.clone());
            borrowed_next_vertex.edges.push(new_edge_rc.clone());
        }

        new_vertexes
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::nodes::graph::Graph;
    use std::f64;
    use std::i8::MIN;
    use std::time::Instant;

    fn create_graph() -> Graph {
        let mut graph = Graph::new("test".to_owned());
        graph.add_vertex("Polevskoy".to_owned());
        graph.add_vertex("Revda".to_owned());
        graph.add_vertex("Pervouralsk".to_owned());
        graph.add_vertex("Ekaterinburg".to_owned());
        graph.add_vertex("Sysert".to_owned());

        graph.connect_vertices("Polevskoy".to_owned(), "Revda".to_owned(), 2f64);
        graph.connect_vertices("Revda".to_owned(), "Pervouralsk".to_owned(), 2f64);
        graph.connect_vertices("Pervouralsk".to_owned(), "Ekaterinburg".to_owned(), 4f64);
        graph.connect_vertices("Polevskoy".to_owned(), "Ekaterinburg".to_owned(), 5f64);
        graph.connect_vertices("Ekaterinburg".to_owned(), "Revda".to_owned(), 3f64);
        graph.connect_vertices("Sysert".to_owned(), "Polevskoy".to_owned(), 3f64);
        graph.connect_vertices("Ekaterinburg".to_owned(), "Sysert".to_owned(), 3f64);

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

        graph.connect_vertices("Верхняя Пышма".to_owned(), "Екатеринбург".to_owned(), 17f64);
        graph.connect_vertices("Березовский".to_owned(), "Екатеринбург".to_owned(), 14f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Дегтярск".to_owned(), 71f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Арамиль".to_owned(), 30f64);
        graph.connect_vertices("Арамиль".to_owned(), "Белоярский".to_owned(), 47f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Заречный".to_owned(), 61f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Полевской".to_owned(), 68f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Ревда".to_owned(), 54f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Асбест".to_owned(), 90f64);
        graph.connect_vertices("Екатеринбург".to_owned(), "Белоярский".to_owned(), 53f64);
        graph.connect_vertices("Верхняя Пышма".to_owned(), "Березовский".to_owned(), 23f64);
        graph.connect_vertices("Заречный".to_owned(), "Березовский".to_owned(), 57f64);
        graph.connect_vertices("Заречный".to_owned(), "Белоярский".to_owned(), 11f64);
        graph.connect_vertices("Асбест".to_owned(), "Заречный".to_owned(), 42f64);
        graph.connect_vertices("Ревда".to_owned(), "Дегтярск".to_owned(), 21f64);
        graph.connect_vertices("Дегтярск".to_owned(), "Полевской".to_owned(), 38f64);
        graph.connect_vertices("Верхняя Пышма".to_owned(), "Первоуральск".to_owned(), 59f64);
        graph.connect_vertices("Первоуральск".to_owned(), "Ревда".to_owned(), 15f64);
        graph.connect_vertices("Сысерть".to_owned(), "Арамиль".to_owned(), 28f64);

        graph
    }

    #[test]
    pub fn generate_random_paths_should_correctly_return_result() {
        let mut graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Polevskoy".to_owned(),
            &"Pervouralsk".to_owned(),
            10,
        );
        let fully_fit_paths = random_routes.iter().count();
        assert!(fully_fit_paths > 0);
    }

    #[test]
    pub fn get_pairs_of_random_routes() {
        let mut graph = create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Polevskoy".to_owned(),
            &"Pervouralsk".to_owned(),
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
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Polevskoy".to_owned(),
            &"Pervouralsk".to_owned(),
            1000,
        );

        let result = processing_algorithm.generate_pairs(&random_routes);
        let crossed = processing_algorithm.crossover(&mut graph.string_pool, &result);
        assert!(crossed.len() > 0);
    }

    #[test]
    pub fn get_fit_value_should_correct_pass_through() {
        let graph = &mut create_graph();
        let processing_algorithm = ProcessingAlgorithm::default();
        let mut time = Instant::now();
        let random_routes = processing_algorithm.generate_random_routes(
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Polevskoy".to_owned(),
            &"Pervouralsk".to_owned(),
            100,
        );

        let elapsed_generation = time.elapsed().as_micros();
        time = Instant::now();
        random_routes.iter().for_each(|r| {
            let fit_value = processing_algorithm.get_fit_value(r);
            assert!(fit_value > 0f64);
        });

        let elapsed_fit = time.elapsed().as_micros();
        println!(
            "Generated in {:?} μs, fitted in {:?} μs",
            elapsed_generation, elapsed_fit
        );
    }

    #[test]
    pub fn crossover_should_work_as_expected() {
        let mut graph = create_complex_graph();

        let processing_algorithm = ProcessingAlgorithm::default();
        let random_routes = processing_algorithm.generate_random_routes(
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Асбест".to_owned(),
            &"Дегтярск".to_owned(),
            1000,
        );

        let pairs = processing_algorithm.generate_pairs(&random_routes);
        let result = processing_algorithm.crossover(&mut graph.string_pool, &pairs);

        assert!(result.len() > 0);
    }

    #[test]
    pub fn should_find_optimal_way_for_complex_graph() {
        let mut graph = create_complex_graph();

        let processing_algorithm = ProcessingAlgorithm::default();
        let mut random_routes = processing_algorithm.generate_random_routes(
            &mut graph.string_pool,
            &graph
                .vertex_references
                .iter()
                .map(|f| Rc::downgrade(&f))
                .collect(),
            &"Сысерть".to_owned(),
            &"Заречный".to_owned(),
            1000,
        );

        let mut crossed = vec![];
        crossed.append(&mut random_routes);

        for _i in 0..100 {
            let pairs = processing_algorithm.generate_pairs(&crossed);
            let _crossed = processing_algorithm.crossover(&mut graph.string_pool, &pairs);

            let mut fit_values = vec![];
            let mut max_fit_value = 0f64;
            for crossed_pair in _crossed {
                let fit = processing_algorithm.get_fit_value(&crossed_pair);
                fit_values.push((fit, crossed_pair));
                if fit > max_fit_value {
                    max_fit_value = fit;
                }
            }

            let rank_value =
                max_fit_value as f64 * (50f64 /* Percentage */ * 0.01f64/* Convert to percents */);
            let mut filter_values: Vec<MutableVertexReferences> = fit_values
                .iter()
                .filter(|f| (f.0 as f64) <= rank_value)
                .map(|(f, v)| v.clone())
                .collect();

            if filter_values.len() > 1 {
                crossed.clear();
                crossed.append(&mut filter_values);
            } else {
                assert!(crossed.len() > 0);
                break;
            }
        }

        assert!(crossed.len() > 0);
    }
}
