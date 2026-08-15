use crate::internal::processing_algorithm::ProcessingAlgorithm;
use crate::internal::types::MutableVertexReferences;
use crate::nodes::graph::Graph;
use std::rc::Rc;

#[must_use = "Main entry point to work with"]
pub struct PathResolver {
    graph: Graph,
    processing_algorithm: ProcessingAlgorithm,
}

impl PathResolver {
    pub fn new(graph_name: String) -> PathResolver {
        PathResolver {
            graph: Graph::new(graph_name),
            processing_algorithm: ProcessingAlgorithm,
        }
    }

    pub fn add_vertex(&mut self, vertex_name: String) {
        self.graph.add_vertex(vertex_name);
    }

    pub fn has_vertex(&self, vertex_name: &str) -> bool {
        self.graph.has_vertex(vertex_name)
    }

    pub fn connect_vertices(
        &mut self,
        source_vector_name: String,
        destination_vector_name: String,
        weight: u32,
    ) {
        self.graph
            .connect_vertices(source_vector_name, destination_vector_name, weight);
    }

    pub fn has_connection(&self, source_vector_name: &String, destination_vector_name: &String) -> bool {
        self.graph.has_connection_between_vertices(
            &source_vector_name,
            &destination_vector_name,
        )
    }

    pub fn resolve_optimal_path(
        &mut self,
        source: &str,
        destination: &str,
        attempts_to_cross: u32,
        amount_of_generated_paths: u32,
        percent_of_fit: u32,
    ) -> Vec<String> {
        let mut random_paths = self.processing_algorithm.generate_random_routes(
            &mut self.graph.string_pool,
            &self
                .graph
                .vertex_references
                .iter()
                .map(Rc::downgrade)
                .collect(),
            &source.to_owned(),
            &destination.to_owned(),
            amount_of_generated_paths,
        );

        let mut crossed = vec![];
        crossed.append(&mut random_paths);

        for _i in 0..attempts_to_cross {
            let pairs = self.processing_algorithm.generate_pairs(&crossed);
            let _crossed = self
                .processing_algorithm
                .crossover(&mut self.graph.string_pool, &pairs);

            let mut fit_values = vec![];
            let mut max_fit_value = u32::MIN;
            for crossed_pair in _crossed {
                let fit = self.processing_algorithm.get_fit_value(&crossed_pair);
                fit_values.push((fit, crossed_pair));
                if fit > max_fit_value {
                    max_fit_value = fit;
                }
            }

            // max_fit_value as f64 * (50f64 /* Percentage */ * 0.01f64 /* Convert to percents */);
            let rank_value = max_fit_value as f64 * (0.01f64 * percent_of_fit as f64);
            let mut filter_values: Vec<MutableVertexReferences> = fit_values
                .iter()
                .filter(|f| f.0 as f64 >= rank_value)
                .map(|(f, v)| v.clone())
                .collect();

            if filter_values.len() > 1 {
                crossed.clear();
                crossed.append(&mut filter_values);
            } else {
                break;
            }
        }

        let first_route = crossed.first().unwrap();
        let mut route = vec![];
        first_route.iter().for_each(|r| {
            let vertex = r.borrow();
            let vertex_name_upgraded = vertex.name.upgrade().unwrap();
            let vertex_name: String = (*vertex_name_upgraded).to_string();
            route.push(vertex_name);
        });

        route
    }
}
