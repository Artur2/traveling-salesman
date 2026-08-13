use crate::nodes::vertex::Vertex;
use crate::upgrade_conditionally;
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

    pub fn has_connection(&self, vertex_identifier: &String) -> bool {
        match (self.source.as_ref(), self.destination.as_ref()) {
            (Some(source), Some(destination)) => {
                let borrowed_source = source.borrow();
                let borrowed_destination = destination.borrow();
                let source_name = upgrade_conditionally!(borrowed_source.name);
                let destination_name = upgrade_conditionally!(borrowed_destination.name);

                source_name.as_ref() == *vertex_identifier
                    || destination_name.as_ref() == *vertex_identifier
            }
            (None, None) => panic!("Cant reach source and destination"),
            _ => panic!(),
        }
    }
}
