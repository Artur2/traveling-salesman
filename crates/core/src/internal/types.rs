use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;

pub(crate) type MutableVertexReferences = Vec<MutableVertexReference>;
pub(crate) type MutableEdgeReferences = Vec<Rc<RefCell<Edge>>>;
pub(crate) type WeakVertexReferences = Vec<WeakVertexReference>;
pub(crate) type MutableVertexReference = Rc<RefCell<Vertex>>;
pub(crate) type MutableEdgeReference = Rc<RefCell<Edge>>;
pub(crate) type WeakVertexReference = Weak<RefCell<Vertex>>;
pub(crate) type WeakEdgeReference = Weak<RefCell<Edge>>;