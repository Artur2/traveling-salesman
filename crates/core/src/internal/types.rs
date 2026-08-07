use crate::nodes::edge::Edge;
use crate::nodes::vertex::Vertex;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub(crate) type MutableVertexReferences = Vec<MutableVertexReference>;
pub(crate) type MutableEdgeReferences = Vec<Rc<RefCell<Edge>>>;
pub(crate) type WeakVertexReferences = Vec<WeakVertexReference>;
pub(crate) type MutableVertexReference = Rc<RefCell<Vertex>>;
pub(crate) type WeakVertexReference = Weak<RefCell<Vertex>>;
