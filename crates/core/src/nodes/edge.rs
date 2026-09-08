use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type OptionalVertexRc = Option<Rc<RefCell<Vertex>>>;

pub(crate) struct Edge {
    pub identifier: Weak<str>,
    pub weight: f64,
    pub source: OptionalVertexRc,
    pub destination: OptionalVertexRc,
}

impl Edge {
    pub fn new(identifier: Weak<str>, weight: f64) -> Self {
        Self {
            identifier,
            weight,
            source: None,
            destination: None
        }
    }
}
