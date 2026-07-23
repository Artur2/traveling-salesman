use std::cell::RefCell;
use std::rc::Rc;
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;

pub type MutableVertexReferences<'a> = Vec<Rc<RefCell<Vertex<'a>>>>;
pub type MutableEdgeReferences<'a> = Vec<Rc<RefCell<Edge<'a>>>>;