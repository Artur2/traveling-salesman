use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

type OptionalVertexRc = Option<Rc<RefCell<Vertex>>>;

#[derive(Default)]
pub(crate) struct Edge {
    pub identifier: String,
    pub weight: u32,
    pub source: OptionalVertexRc,
    pub destination: OptionalVertexRc,
}

impl Edge {
    pub fn new(identifier: String, weight: u32) -> Self {
        Self {
            identifier,
            weight,
            ..Default::default()
        }
    }
}
