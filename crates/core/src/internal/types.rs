use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;

pub(crate) type MutableVertexReferences = Vec<Rc<RefCell<Vertex>>>;
pub(crate) type MutableEdgeReferences = Vec<Rc<RefCell<Edge>>>;
pub(crate) type WeakVertexReferences = Vec<Weak<RefCell<Vertex>>>;