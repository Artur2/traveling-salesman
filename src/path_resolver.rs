use crate::graph::Graph;
use crate::processing_algorithm::ProcessingAlgorithm;

#[must_use = "Main entry point to work with"]
pub struct PathResolver<'a> {
    graph: Graph<'a>,
    processing_algorithm: ProcessingAlgorithm,
}

impl<'a> PathResolver<'a> {
    pub fn new(graph_name: &'a str) -> PathResolver<'a> {
        PathResolver {
            graph: Graph::new(graph_name),
            processing_algorithm: ProcessingAlgorithm::default(),
        }
    }

    #[must_use = "to resolve path between vertices"]
    pub fn add_vertex(&mut self, vertex_name: &'a str) {
        self.graph.add_vertex(vertex_name);
    }

    #[must_use = "to add weighted edges between vertices"]
    pub fn connect_vertices(
        &mut self,
        source_vector_name: &str,
        destination_vector_name: &str,
        edge_identifier: &'a str,
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

        todo!("Implement optimal path")
    }
}
