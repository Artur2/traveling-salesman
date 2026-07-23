use std::cell::RefCell;
use std::rc::Rc;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;

pub(crate) type MutableVertexReferences<'a> = Vec<Rc<RefCell<Vertex<'a>>>>;
pub(crate) type MutableEdgeReferences<'a> = Vec<Rc<RefCell<Edge<'a>>>>;