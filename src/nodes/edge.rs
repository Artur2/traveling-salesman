use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;

type OptionalVertexRc<'a> = Option<Rc<RefCell<Vertex<'a>>>>;

#[derive(Default)]
pub struct Edge<'a> {
    pub identifier: &'a str,
    pub weight: u32,
    pub source: OptionalVertexRc<'a>,
    pub destination: OptionalVertexRc<'a>,
}

impl<'a> Edge<'a> {
    pub fn new(identifier: &'a str, weight: u32) -> Self {
        Self {
            identifier,
            weight,
            ..Default::default()
        }
    }
}
