use crate::nodes::edge::Edge;
use crate::upgrade_conditionally;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};
use crate::internal::string_pool::StringPool;

pub(crate) struct Vertex {
    pub name: Weak<str>,
    pub edges: Vec<Rc<RefCell<Edge>>>,
}

impl Vertex {
    pub fn new(name: Weak<str>) -> Self {
        Vertex {
            name,
            edges: vec![],
        }
    }

    pub fn has_connection(&self, destination_vector_name: &str) -> bool {
        self.edges.iter().any(|edge| {
            let borrowed_edge = edge.borrow();
            let mut has = false;

            if let Some(destination) = &borrowed_edge.destination {
                let borrowed_vector = destination.borrow();
                let upgraded_name = upgrade_conditionally!(borrowed_vector.name);
                has |= upgraded_name.as_ref() == destination_vector_name;
            }

            if let Some(source) = &borrowed_edge.source {
                let borrowed_vector = source.borrow();
                let upgraded_name = upgrade_conditionally!(borrowed_vector.name);
                has |= upgraded_name.as_ref() == destination_vector_name;
            }

            has
        })
    }

    pub fn add_connection(
        string_pool: &mut StringPool,
        source_vertex: &Rc<RefCell<Vertex>>,
        destination_vertex: &Rc<RefCell<Vertex>>,
        weight: u32,
    ) {
        let mut edge_name: String = Default::default();
        let mut source_vertex_name: String = Default::default();
        let mut destination_vertex_name: String = Default::default();
        {
            let borrowed_source_vertex = source_vertex.borrow();
            let borrowed_destination_vertex = destination_vertex.borrow();
            let source_name = upgrade_conditionally!(borrowed_source_vertex.name);
            let destination_name = upgrade_conditionally!(borrowed_destination_vertex.name);
            source_vertex_name += source_name.as_ref();
            destination_vertex_name += destination_name.as_ref();

            edge_name = format!("{}-{}", source_vertex_name, destination_vertex_name);
        }

        let mut new_edge = Edge::new(string_pool.intern(edge_name), weight);
        new_edge.source = Some(source_vertex.clone());
        new_edge.destination = Some(destination_vertex.clone());

        let new_edge_rc = Rc::new(RefCell::new(new_edge));

        let mut source_modify = false;
        let mut destination_modify = false;
        {
            if !source_vertex
                .borrow()
                .has_connection(&destination_vertex_name)
            {
                source_modify = true;
            }
            if !destination_vertex
                .borrow()
                .has_connection(&source_vertex_name)
            {
                destination_modify = true;
            }
        }

        if source_modify {
            let mut mutable_vertex = source_vertex.borrow_mut();
            mutable_vertex.edges.push(new_edge_rc.clone());
        }
        if destination_modify {
            let mut mutable_vertex = destination_vertex.borrow_mut();
            mutable_vertex.edges.push(new_edge_rc.clone());
        }
    }
}
