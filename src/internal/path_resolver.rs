use crate::graph::Graph;
use crate::internal::processing_algorithm::ProcessingAlgorithm;

#[must_use = "Main entry point to work with"]
pub struct PathResolver {
    graph: Graph,
    processing_algorithm: ProcessingAlgorithm,
}

impl PathResolver {
    pub fn new(graph_name: String) -> PathResolver {
        PathResolver {
            graph: Graph::new(graph_name),
            processing_algorithm: ProcessingAlgorithm::default(),
        }
    }

    #[must_use = "to resolve path between vertices"]
    pub fn add_vertex(&mut self, vertex_name: String) {
        self.graph.add_vertex(vertex_name);
    }

    #[must_use = "to add weighted edges between vertices"]
    pub fn connect_vertices(
        &mut self,
        source_vector_name:String,
        destination_vector_name: String,
        edge_identifier: String,
        weight: u32,
    ) {
        self.graph.connect_vertices(
            source_vector_name,
            destination_vector_name,
            edge_identifier,
            weight,
        );
    }

    #[must_use = "to get result of GA algorithm to get optimal way through vertices/edges"]
    pub fn resolve_optimal_path(&self, source: &str, destination: &str) -> Vec<&str> {
        let random_paths = self.processing_algorithm.generate_random_routes(
            &self.graph.vertex_references,
            source,
            destination,
            100,
        );
        let pairs = self.processing_algorithm.generate_pairs(&random_paths);
        let crossed = self.processing_algorithm.crossover(pairs);

        todo!("Implement optimal path")
    }
}
