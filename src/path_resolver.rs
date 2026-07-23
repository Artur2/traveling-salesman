use crate::graph::Graph;
use crate::processing_algorithm::ProcessingAlgorithm;

pub struct PathResolver<'a> {
    pub graph: Graph<'a>,
    pub processing_algorithm: ProcessingAlgorithm,
}

impl<'a> PathResolver<'a> {
    pub fn new(graph_name: &'a str) -> PathResolver<'a> {
        PathResolver {
            graph: Graph::new(graph_name),
            processing_algorithm: ProcessingAlgorithm::default(),
        }
    }
}
