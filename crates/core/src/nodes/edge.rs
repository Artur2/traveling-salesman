use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

    pub fn has_connection(&self, vertex_identifier: &String) -> bool {
        if let (Some(source), Some(destination)) = (self.source.as_ref(), self.destination.as_ref())
        {
            let borrowed_source = source.borrow();
            let borrowed_destination = destination.borrow();

            return borrowed_source.name == *vertex_identifier
                || borrowed_destination.name == *vertex_identifier;
        }

        false
    }
}
