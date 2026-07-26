use crate::nodes::graph::Graph;
use crate::nodes::edge::Edge;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Default)]
pub(crate) struct Vertex {
    pub name: String,
    pub edges: Vec<Weak<RefCell<Edge>>>,
}

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut formatted = format!("{} with edges: ", self.name);
        for (i, edge) in self.edges.iter().enumerate() {
            match edge.upgrade() {
                None => {}
                Some(edge) => {
                    formatted += format!("{}", edge.borrow().identifier).as_str();
                }
            }
        }

        write!(f, "{}", formatted)
    }
}

impl Vertex {
    pub fn new(name: String) -> Self {
        Vertex {
            name,
            edges: vec![],
        }
    }

    pub fn has_connection(&self, destination_vector_name: &str) -> bool {
        self.edges.iter().any(|edge| match edge.upgrade() {
            Some(edge) => {
                let borrowed_edge = edge.borrow();
                let mut has = false;

                if let Some(destination) = &borrowed_edge.destination {
                    match destination.upgrade() {
                        None => panic!("Cannot reach out edge's destination"),
                        Some(destination) => {
                            let borrowed_vector = destination.borrow();
                            has |= borrowed_vector.name == destination_vector_name;
                        }
                    }
                }

                if let Some(source) = &borrowed_edge.source {
                    match source.upgrade() {
                        None => panic!("Cannot reach out edge's destination"),
                        Some(source) => {
                            let borrowed_vector = source.borrow();
                            has |= borrowed_vector.name == destination_vector_name;
                        }
                    }
                }

                return has;
            }
            None => false,
        })
    }

    #[allow(unused_assignments)]
    pub fn add_connection(
        graph: &mut Graph,
        source_vertex: &Weak<RefCell<Vertex>>,
        destination_vertex: &Weak<RefCell<Vertex>>,
        weight: u32,
    ) {
        let mut edge_name: String = Default::default();
        let mut source_vertex_name: String = Default::default();
        let mut destination_vertex_name: String = Default::default();
        {
            let destination_vertex_v2 = destination_vertex.upgrade();
            let source_vertex_v2 = source_vertex.upgrade();
            match source_vertex_v2 {
                None => {}
                Some(vertex) => {
                    source_vertex_name += vertex.borrow().name.as_str();
                }
            }
            match destination_vertex_v2 {
                None => {}
                Some(vertex) => {
                    destination_vertex_name += vertex.borrow().name.as_str();
                }
            }

            edge_name = format!("{}-{}", source_vertex_name, destination_vertex_name);
        }

        let existing = graph.edge_references.iter().find(|p| {
            let borrowed = p.borrow();
            borrowed.identifier == edge_name
        });

        let mut new_edge = Edge::new(edge_name, weight);
        new_edge.source = Some(source_vertex.clone());
        new_edge.destination = Some(destination_vertex.clone());

        let edge_rc = if existing.is_none() {
            let new_edge = RefCell::new(new_edge);
            let edge_rc = Rc::new(new_edge);
            graph.edge_references.push(edge_rc.clone());
            Rc::downgrade(&edge_rc)
        } else {
            Rc::downgrade(&existing.unwrap())
        };

        let mut source_modify = false;
        let mut destination_modify = false;
        {
            let upgraded_source_vertex = source_vertex.upgrade();
            let upgraded_destination_vertex = destination_vertex.upgrade();

            match upgraded_source_vertex {
                Some(source_vertex) => {
                    if !source_vertex
                        .borrow()
                        .has_connection(&destination_vertex_name)
                    {
                        source_modify = true;
                    }
                }
                None => panic!("Cant reach out vertex"),
            }
            match upgraded_destination_vertex {
                Some(destination_vertex) => {
                    if !destination_vertex
                        .borrow()
                        .has_connection(&source_vertex_name)
                    {
                        destination_modify = true;
                    }
                }
                None => panic!("Cant reach out vertex"),
            }
        }

        if source_modify {
            let upgraded_source_vertex = source_vertex.upgrade();
            match upgraded_source_vertex {
                None => panic!("Cant upgrade source vertex"),
                Some(vertex) => {
                    let mut mutable_vertex = vertex.borrow_mut();
                    mutable_vertex.edges.push(edge_rc.clone());
                }
            }
        }
        if destination_modify {
            let upgraded_destination_vertex = destination_vertex.upgrade();
            match upgraded_destination_vertex {
                None => panic!("Cant reach out vertex"),
                Some(vertex) => {
                    let mut mutable_vertex = vertex.borrow_mut();
                    mutable_vertex.edges.push(edge_rc.clone());
                }
            }
        }
    }
}
