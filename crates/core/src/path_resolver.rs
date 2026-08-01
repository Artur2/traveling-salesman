use crate::nodes::graph::Graph;
use crate::internal::processing_algorithm::ProcessingAlgorithm;
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
    
    pub fn connect_vertices(
        &mut self,
        source_vector_name: String,
        destination_vector_name: String,
        weight: u32,
    ) {
        self.graph
            .connect_vertices(source_vector_name, destination_vector_name, weight);
    }
    
    pub fn resolve_optimal_path(&mut self, source: &str, destination: &str, _attempts_to_cross: u32, amount_of_generated_paths: u32) -> Vec<&str> {
        let random_paths = self.processing_algorithm.generate_random_routes(
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
        // let pairs = self.processing_algorithm.generate_pairs(&random_paths);
        // let _crossed = self.processing_algorithm.crossover(&mut self.graph, pairs);

        todo!("Implement optimal path")
    }
}
